use clap::Command;

use crate::Runner;

#[derive(Debug, Clone)]
pub enum PartialFitType {}

impl Runner for PartialFitType {
    fn run(&self, cli: &crate::Cli) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

pub const CMD_PARTIAL_FIT: &str = "partial-fit";

pub fn command() -> Command {
    Command::new(CMD_PARTIAL_FIT)
        .short_flag('f')
        .about("Partial [f]it ML algorithms class")
}
