use clap::Parser;
use notify_rust::{Notification, Timeout};
use std::process::Command;
use std::time::{Duration, Instant};

pub struct App {}

impl App {
    pub fn run() {
        let opts: Opts = Opts::parse();

        let command_string = opts.expression.join(" ");

        if execute(command_string.as_str()).as_secs() > opts.duration {
            let mut notification = Notification::new();
            notification
                .summary("Command finished")
                .body(command_string.as_str())
                .sound_name(SOUND)
                .timeout(Timeout::Milliseconds(2000));

            notification.show().unwrap();
        }
    }
}

/// Notify when a long-running command has finished
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    // Duration (in seconds) after which a notification is triggered upon command completion
    #[clap(short, long, default_value_t = 60)]
    duration: u64,

    // Command to be executed
    #[clap(last = true)]
    expression: Vec<String>,
}

fn execute(command: &str) -> Duration {
    let before = Instant::now();
    let spawn_result = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["-Command", command])
            .spawn()
    } else {
        let shell: String = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
        Command::new(shell).arg("-c").arg(command).spawn()
    };

    spawn_result.and_then(|mut child| child.wait()).unwrap();
    Instant::now().duration_since(before)
}

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instant";

#[cfg(target_os = "windows")]
static SOUND: &'static str = "Mail";
