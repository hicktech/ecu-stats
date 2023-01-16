use clap::Parser;

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

    /// Record number of events before exiting
    #[clap(short, long)]
    pub count: Option<usize>,
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
    #[clap(long, default_value = "data/j1939_utf8.dbc")]
    pub dbc: String,

    #[clap(default_value = "vcan0")]
    pub socket: String,

    #[clap(short, long)]
    pub journal: String,
}

#[derive(Parser)]
pub struct CountOpts {
    #[clap(long, default_value = "data/j1939_utf8.dbc")]
    pub dbc: String,

    #[clap(short, long)]
    pub journal: String,
}
