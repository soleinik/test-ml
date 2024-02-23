use std::error::Error;

use clap::{ArgMatches, Command};
use log::error;

use crate::Runner;

mod partial_fit;
mod pre_processor;
mod supervised;
mod unsupervised;

pub const CMD_ALGO_CATEGORY: &str = "AlgoCategory";
pub fn command() -> Command {
    let cmd_supervised = supervised::command();
    let cmd_unsupervised = unsupervised::command();
    let cmd_partial_fit = partial_fit::command();
    let cmd_preprocessor = pre_processor::command();

    Command::new(CMD_ALGO_CATEGORY)
        .short_flag('C')
        .about("Algorithm's [C]ategory")
        .subcommand_required(true)
        .subcommand(cmd_supervised)
        .subcommand(cmd_unsupervised)
        .subcommand(cmd_partial_fit)
        .subcommand(cmd_preprocessor)
}

#[derive(Debug, Clone)]
pub enum AlgoCategory {
    Supervised {
        sub: supervised::SupervisedType,
    },
    UnSupervised {
        sub: unsupervised::UnsupervisedType,
    },
    PreProcessor {
        sub: pre_processor::PreProcessorType,
    },
    PartialFit {
        sub: partial_fit::PartialFitType,
    },
}

impl Runner for AlgoCategory {
    fn run(&self, cli: &crate::Cli) -> Result<(), Box<dyn Error>> {
        match self {
            AlgoCategory::Supervised { sub } => sub as &dyn Runner,
            AlgoCategory::UnSupervised { sub } => sub as &dyn Runner,
            AlgoCategory::PreProcessor { sub } => sub as &dyn Runner,
            AlgoCategory::PartialFit { sub } => sub as &dyn Runner,
        }
        .run(cli)
    }
}

impl TryFrom<&ArgMatches> for AlgoCategory {
    type Error = String;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let Some((cmd_name, nxt_matches)) = matches.subcommand() else {
            //todo - throw exception
            panic!("todo-01 - can't find sub command");
        };

        match cmd_name {
            supervised::CMD_SUPERVISED => Ok(AlgoCategory::Supervised {
                sub: supervised::SupervisedType::try_from(nxt_matches)?,
            }),
            unsupervised::CMD_UNSUPERVISED => Ok(AlgoCategory::UnSupervised {
                sub: unsupervised::UnsupervisedType::try_from(nxt_matches)?,
            }),
            pre_processor::CMD_PRE_PROCESSOR => {
                // Ok(AlgoCategory::PreProcessor {
                //     sub: pre_processor::PreProcessorGroup::try_from(matches)?,
                // })
                unimplemented!()
            }
            partial_fit::CMD_PARTIAL_FIT => {
                // Ok(AlgoCategory::PartialFit {
                //     sub: partial_fit::PartialFitGroup::try_from(matches)?,
                // }
                // )
                unimplemented!()
            }
            _ => {
                error!("Algorithm Category \"{cmd_name}\" is not supported");
                std::process::exit(1);
            }
        }
    }
}
