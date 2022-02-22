use std::process::Command;
use std::time::{Duration, Instant};

pub(crate) fn execute(command: &[String]) -> Duration {
    let command_base = command.first().unwrap();
    let args: Vec<String> = command
        .iter()
        .enumerate()
        .filter_map(|(index, arg)| if index > 0 { Some(arg) } else { None })
        .cloned()
        .collect();

    let before = Instant::now();
    let spawn_result = Command::new(command_base).args(args).spawn();

    spawn_result.and_then(|mut child| child.wait()).unwrap();
    Instant::now().duration_since(before)
}
