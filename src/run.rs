use canparse::pgn::PgnLibrary;
use clap::Parser;
use j1939_stats::cli::{DumpOpts, Opts, PlaybackOpts, RecordingOpts, Subcommand};
use j1939_stats::{is_proprietary_pgn, pgn_from_dbc};
use socketcan::CANFrame;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn record(opts: RecordingOpts) {
    let can = socketcan::CANSocket::open(&opts.socket).expect("open can");
    let db: sled::Db = sled::open(opts.journal).unwrap();

    eprintln!("Recording ...");

    let mut i = 0;
    loop {
        let k = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
            .to_be_bytes();

        // format: [id][data]
        let f = can.read_frame().unwrap();
        let id = f.id().to_be_bytes();
        let v = [id.as_slice(), f.data()].concat();
        db.insert(k, v).unwrap();

        i += 1;
        eprint!("{i}\r");

        if let Some(c) = opts.count {
            if !(i < c) {
                break;
            }
        }
    }

    eprintln!("Recorded {i} events");
}

fn playback(opts: PlaybackOpts) {
    let can = socketcan::CANSocket::open(&opts.socket).expect("open can");
    let db: sled::Db = sled::open(opts.journal).unwrap();
    for e in db.iter() {
        let (_, b) = e.unwrap();
        let (id, data) = b.split_at(std::mem::size_of::<u32>());
        let id = u32::from_be_bytes(id.try_into().unwrap());

        let frame = CANFrame::new(id, data, false, false).unwrap();
        can.write_frame(&frame).unwrap();

        sleep(Duration::from_millis(opts.delay));
    }
}

fn dump(opts: DumpOpts) {
    let db: sled::Db = sled::open(opts.journal).unwrap();

    let lib = PgnLibrary::from_dbc_file(opts.dbc).expect("open dbc");
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

fn main() {
    let all_opts: Opts = Opts::parse();

    match all_opts.cmd {
        Subcommand::Rec(opts) => record(opts),
        Subcommand::Play(opts) => playback(opts),
        Subcommand::Dump(opts) => dump(opts),
    }
}
