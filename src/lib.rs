use std::ops::Mul;

use anyhow::{Context, Result};
use clap::Parser;
use notify_rust::{Notification, Timeout};

use execute::execute;

use crate::command::Command;
use crate::opts::Opts;

mod command;
mod execute;
mod opts;

pub struct App {}

impl App {
    pub fn run() -> Result<()> {
        let opts: Opts = Opts::parse();

        if opts.command.is_empty() {
            return Ok(());
        }

        let command = Command::try_from(opts.command)?;

        if execute(&command)?.as_secs() > opts.timeout_period {
            let mut notification = Notification::new();
            notification
                .summary("Command finished")
                .body(command.to_string().as_str())
                .sound_name(SOUND)
                .timeout(Timeout::Milliseconds(opts.display_period.mul(1000) as u32));

            notification.show().context("Failed to show notification")?;
        }

        Ok(())
    }
}

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instant";

#[cfg(target_os = "windows")]
static SOUND: &'static str = "Mail";
