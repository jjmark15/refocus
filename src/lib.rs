use clap::Parser;
use notify_rust::{Notification, Timeout};

use execute::execute;

use crate::opts::Opts;

mod execute;
mod opts;

pub struct App {}

impl App {
    pub fn run() {
        let opts: Opts = Opts::parse();

        let command = opts.command;

        if execute(&command).as_secs() > opts.duration {
            let mut notification = Notification::new();
            notification
                .summary("Command finished")
                .body(command.join(" ").as_str())
                .sound_name(SOUND)
                .timeout(Timeout::Milliseconds(2000));

            notification.show().unwrap();
        }
    }
}

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instant";

#[cfg(target_os = "windows")]
static SOUND: &'static str = "Mail";
