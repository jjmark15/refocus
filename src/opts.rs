use clap::Parser;

/// Notify when a long-running command has finished
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Opts {
    /// Period (in seconds) after which a notification is triggered upon command completion
    #[clap(short, long, default_value_t = 60)]
    pub(crate) timeout_period: u64,

    /// Command to be executed
    #[clap(last = true)]
    pub(crate) command: Vec<String>,

    /// Period (in seconds) to display the notification
    #[clap(short, long, default_value_t = 2)]
    pub(crate) display_period: u64,
}
