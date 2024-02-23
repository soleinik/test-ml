use clap::Command;

use crate::Runner;

#[derive(Debug, Clone)]
pub enum PreProcessorType {}

impl Runner for PreProcessorType {
    fn run(&self, cli: &crate::Cli) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

pub const CMD_PRE_PROCESSOR: &str = "pre-processor";

pub fn command() -> Command {
    Command::new(CMD_PRE_PROCESSOR)
        .short_flag('p')
        .about("[p]re Processors ML algorithms class")
}
