use clap::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[clap(name = "J1939 Stats")]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Parser)]
pub enum Subcommand {
    /// Record CAN messages to journal
    Rec(RecordingOpts),
    /// Replay CAN messages from journal
    Play(PlaybackOpts),
    /// Display J1939 messages from journal
    Dump(DumpOpts),
    /// Count events from journal
    Count(CountOpts),
}

#[derive(Parser)]
pub struct RecordingOpts {
    #[clap(default_value = "vcan0")]
    pub socket: String,

    #[clap(short, long)]
    pub journal: String,

    /// Limit number of recorded events
    #[clap(short, long)]
    pub limit: Option<usize>,

    /// Compression factor (1-22)
    /// Uses zstd compression.
    /// Ranges from 1 up to 22. Levels >= 20 are ‘ultra’.
    #[clap(short, long)]
    pub compression: Option<i32>,
}

#[derive(Parser)]
pub struct PlaybackOpts {
    #[clap(long, default_value = "data/j1939_utf8.dbc")]
    pub dbc: String,

    #[clap(default_value = "vcan0")]
    pub socket: String,

    #[clap(short, long)]
    pub journal: String,

    /// Millisecond delay between events
    #[clap(short, long, default_value = "0")]
    pub delay: u64,
}

#[derive(Parser)]
pub struct DumpOpts {
    pub from: DumpType,

    #[clap(long, default_value = "data/j1939_utf8.dbc")]
    pub dbc: String,

    #[clap(default_value = "vcan0")]
    pub socket: String,

    #[clap(short, long)]
    pub journal: String,
}

#[derive(Debug, Clone)]
pub enum DumpType {
    All,
    PGNs,
}

#[derive(Parser)]
pub struct CountOpts {
    #[clap(long, default_value = "data/j1939_utf8.dbc")]
    pub dbc: String,

    #[clap(short, long)]
    pub journal: String,

    /// Filter by PGN
    pub pgn: Option<u32>,
}

impl FromStr for DumpType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(DumpType::All),
            "pgns" => Ok(DumpType::PGNs),
            x => Err(format!("{} is not supported", x)),
        }
    }
}
