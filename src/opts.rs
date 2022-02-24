use clap::Parser;

/// Notify when a long-running command has finished
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Opts {
    /// Command to be executed
    #[clap(last = true)]
    pub(crate) command: Vec<String>,
}
