use std::error::Error;

use clap::{ArgMatches, Command};
use log::error;

use crate::Runner;

mod support_vector_machines;

#[derive(Debug, Clone)]
pub enum SupportVectorMachines {
    SVM { eps: f64 },
}

impl Runner for SupportVectorMachines {
    fn run(&self, _cli: &crate::Cli) -> Result<(), Box<dyn Error>> {
        match self {
            SupportVectorMachines::SVM { eps } => {
                ml_lib::preamble::svm::doit("./data/IBM-5y.csv", "./data/^GSPC-5y.csv", *eps)
            }
        }
    }
}

impl TryFrom<&ArgMatches> for SupportVectorMachines {
    type Error = String;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let Some((cmd_name, nxt_matches)) = matches.subcommand() else {
            //todo - throw exception
            panic!("todo-01 - can't find sub command");
        };

        match cmd_name {
            support_vector_machines::CMD_SUPPORT_VECTOR_MACHINES => {
                let Some(eps) = nxt_matches.get_one::<f64>(support_vector_machines::ARG_EPS) else {
                    //this should never happen - we provide arg's default
                    panic!("this should never happen - we provide default eps value for SVM")
                };

                Ok(SupportVectorMachines::SVM {
                    eps: eps.to_owned(),
                })
            }
            _ => {
                error!("Algorithm \"{cmd_name}\" is not supported");
                std::process::exit(1);
            }
        }
    }
}

pub const CMD_SVM: &str = "SVM";

pub fn command() -> Command {
    let cmd_svm = support_vector_machines::command();

    Command::new(CMD_SVM)
        .short_flag('s')
        .about("[s]upport Vector Machines")
        .long_about("Support Vector Machines are a major branch of machine learning models and offer\
         classification or regression analysis of labeled datasets. They seek a discriminant, which\
         separates the data in an optimal way, e.g. have the fewest numbers of miss-classifications and\
         maximizes the margin between positive and negative classes. A support vector contributes to the\
         discriminant and is therefore important for the classification/regression task. The balance\
         between the number of support vectors and model performance can be controlled with hyperparameters.")
         .subcommand_required(true)
         .subcommand(cmd_svm)
}
