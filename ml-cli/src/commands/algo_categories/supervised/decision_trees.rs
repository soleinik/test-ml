use std::error::Error;

use clap::{ArgMatches, Command};
use log::error;

use crate::Runner;

mod random_forest;

/*
Some techniques, often called ensemble methods, construct more than one decision tree:

    Boosted trees Incrementally building an ensemble by training each new instance to emphasize the training instances previously mis-modeled. A typical example is AdaBoost. These can be used for regression-type and classification-type problems.[8][9]
    Bootstrap aggregated (or bagged) decision trees, an early ensemble method, builds multiple decision trees by repeatedly resampling training data with replacement, and voting the trees for a consensus prediction.[10]
        A random forest classifier is a specific type of bootstrap aggregating
    Rotation forest – in which every decision tree is trained by first applying principal component analysis (PCA) on a random subset of the input features.


*/
#[derive(Debug, Clone)]
pub enum DecisionTrees {
    RandomForest,
}

impl Runner for DecisionTrees {
    fn run(&self, _cli: &crate::Cli) -> Result<(), Box<dyn Error>> {
        match self {
            DecisionTrees::RandomForest => {
                ml_lib::preamble::random_forest::doit("./data/IBM-5y.csv", "./data/^GSPC-5y.csv")
            }
        }
    }
}

impl TryFrom<&ArgMatches> for DecisionTrees {
    type Error = String;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let Some((cmd_name, _nxt_matches)) = matches.subcommand() else {
            //todo - throw exception
            panic!("todo-01 - can't find sub command");
        };

        match cmd_name {
            random_forest::CMD_RANDOM_FOREST => Ok(DecisionTrees::RandomForest),
            _ => {
                error!("Algorithm \"{cmd_name}\" is not supported");
                std::process::exit(1);
            }
        }
    }
}

pub const CMD_DECISION_TREE: &str = "DT";

pub fn command() -> Command {
    let cmd_random_forest = random_forest::command();

    Command::new(CMD_DECISION_TREE)
        .short_flag('d')
        .about("[d]ecison trees learning algorithm")
        .long_about(
            "Decision Trees (DTs) are a non-parametric supervised learning method used for classification and regression.\n\
            The goal is to create a modelthat predicts the value of a target variable by learning simple decision rules\n\
            inferred from the data features.",
        )
         .subcommand_required(true)
         .subcommand(cmd_random_forest)
}
