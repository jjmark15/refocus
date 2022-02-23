use anyhow::Context;
use std::time::{Duration, Instant};

use crate::command::Command;

pub(crate) fn execute(command: &Command) -> anyhow::Result<Duration> {
    let before = Instant::now();
    let spawn_result = std::process::Command::new(command.executable())
        .args(command.args())
        .spawn();

    spawn_result
        .and_then(|mut child| child.wait())
        .context(format!("Could not execute '{}'", command))?;
    Ok(Instant::now().duration_since(before))
}
