use canparse::pgn::PgnLibrary;
use clap::Parser;
use ecustats::cli::Subcommand::*;
use ecustats::cli::*;
use ecustats::{is_proprietary_pgn, pgn_from_dbc};
use signal_hook::consts::SIGINT;
use socketcan::{CANFrame, ShouldRetry};
use std::error::Error;
use std::io::ErrorKind::Interrupted;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

type Res = Result<(), Box<dyn Error>>;
const PID_SZ: usize = std::mem::size_of::<u32>();

fn record(opts: RecordingOpts) -> Res {
    let can = socketcan::CANSocket::open(&opts.socket).expect("open can");
    can.set_read_timeout(Duration::from_secs(1)).unwrap();

    let db = if let Some(c) = opts.compression {
        sled::Config::default()
            .path(opts.journal)
            .use_compression(true)
            .compression_factor(c)
            .open()
            .unwrap()
    } else {
        sled::open(opts.journal).unwrap()
    };

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(SIGINT, Arc::clone(&term))?;

    eprint!("Recording ...");

    let mut i = 0;
    while !term.load(Ordering::Relaxed) {
        let f = match can.read_frame() {
            Ok(f) => f,
            Err(e) if e.should_retry() => continue,
            Err(e) if e.kind() == Interrupted => continue,
            Err(e) => return Err(Box::new(e)),
        };

        let k = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
            .to_be_bytes();

        let fid = f.id();
        let cid = fid.to_be_bytes();
        let pid = pgn_from_dbc(f.id());

        // [t] -> [id][data]
        let v = [cid.as_slice(), f.data()].concat();
        db.insert(k, &*v).unwrap();

        // [pid] -> [t] -> [id][data]
        let tree = db.open_tree(pid.to_be_bytes()).unwrap();
        tree.insert(k, &*v).unwrap();

        i += 1;
        eprint!("\rRecording {i}\r");

        if let Some(c) = opts.limit {
            if i >= c {
                break;
            }
        }
    }

    eprintln!(
        "\rRecorded {i} events across {} PGNs",
        db.tree_names().iter().count()
    );
    Ok(())
}

fn playback(opts: PlaybackOpts) -> Res {
    let can = socketcan::CANSocket::open(&opts.socket).expect("open can");
    let db: sled::Db = sled::open(opts.journal).unwrap();
    for e in db.iter() {
        let (_, b) = e.unwrap();
        let (id, data) = b.split_at(PID_SZ);
        let id = u32::from_be_bytes(id.try_into().unwrap());

        let frame = CANFrame::new(id, data, false, false).unwrap();
        can.write_frame(&frame).unwrap();

        sleep(Duration::from_millis(opts.delay));
    }

    Ok(())
}

fn dump(opts: DumpOpts) -> Res {
    let lib = PgnLibrary::from_dbc_file(opts.dbc).expect("open dbc");
    let db: sled::Db = sled::open(opts.journal).unwrap();
    match opts.from {
        DumpType::All => dump_all(&db, &lib),
        DumpType::PGNs => dump_pgns(&db, &lib),
    };
    Ok(())
}

fn dump_all(db: &sled::Db, lib: &PgnLibrary) {
    for e in db.iter() {
        let (t, b) = e.unwrap();
        let (id, _) = b.split_at(std::mem::size_of::<u32>());
        let id = u32::from_be_bytes(id.try_into().unwrap());

        let time = u128::from_be_bytes((&*t).try_into().unwrap());
        let pgn_id = pgn_from_dbc(id);
        match lib.get_pgn(pgn_id) {
            Some(pgn) => println!("{} {}", time, pgn.description),
            None if !is_proprietary_pgn(pgn_id) => eprintln!("Unknown: {pgn_id}"),
            _ => {}
        }
    }
}

fn dump_pgns(db: &sled::Db, lib: &PgnLibrary) {
    for id in db.tree_names().iter().filter(|n| n.len() == PID_SZ) {
        let pid = u32::from_be_bytes(id.as_ref().try_into().unwrap());
        match lib.get_pgn(pid) {
            Some(pgn) => println!("{}", pgn.description),
            None if !is_proprietary_pgn(pid) => eprintln!("Unknown: {pid}"),
            _ => {}
        }
    }
}

fn count(opts: CountOpts) -> Res {
    let db = sled::open(opts.journal).unwrap();
    let c = match opts.pgn {
        Some(id) => db.open_tree(id.to_be_bytes()).unwrap().iter().count(),
        None => db.iter().count(),
    };
    println!("{c}");
    Ok(())
}

fn main() -> Res {
    let all_opts: Opts = Opts::parse();

    match all_opts.cmd {
        Rec(opts) => record(opts)?,
        Play(opts) => playback(opts)?,
        Dump(opts) => dump(opts)?,
        Count(opts) => count(opts)?,
    }

    Ok(())
}
