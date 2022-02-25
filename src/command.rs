use std::fmt::{Display, Formatter};

use anyhow::Context;

#[derive(derive_getters::Getters)]
pub(crate) struct Command {
    executable: String,
    args: Vec<String>,
}

impl TryFrom<Vec<String>> for Command {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let executable = value
            .first()
            .cloned()
            .context("Command does not contain an executable")?;
        let args: Vec<String> = value
            .iter()
            .enumerate()
            .filter_map(|(index, arg)| if index > 0 { Some(arg) } else { None })
            .cloned()
            .collect();
        Ok(Command { executable, args })
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.args.is_empty() {
            write!(f, "{} {}", self.executable, self.args.join(" "))
        } else {
            write!(f, "{}", self.executable)
        }
    }
}
