use clap::Parser;
use notify_rust::{Notification, Timeout};
use std::process::Command;
use std::time::{Duration, Instant};

pub struct App {}

impl App {
    pub fn run() {
        let opts: Opts = Opts::parse();

        let command_string = opts.expression.join(" ");

        let duration = execute(command_string.as_str());

        if duration.as_secs() > 3 {
            let mut notification = Notification::new();
            notification
                .summary("Command finished")
                .body(command_string.as_str())
                .icon("utilities-terminal")
                .timeout(Timeout::Milliseconds(2000));

            notification.show().unwrap();
        }
    }
}

#[derive(Debug, Parser)]
struct Opts {
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
