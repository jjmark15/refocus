use clap::Parser;

/// Notify when a long-running command has finished
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Opts {
    /// Duration (in seconds) after which a notification is triggered upon command completion
    #[clap(short, long, default_value_t = 60)]
    pub(crate) duration: u64,

    /// Command to be executed
    #[clap(last = true)]
    pub(crate) command: Vec<String>,
}
