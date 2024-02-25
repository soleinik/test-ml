use std::error::Error;

use clap::{ArgMatches, Command};
use log::error;

use crate::Runner;

mod bayes;
mod decision_trees;
mod elastic_net;
mod pls;
mod svm;

/*
    Support-vector machines
    Linear regression
    Logistic regression
    Naive Bayes
    Linear discriminant analysis
    Decision trees
    K-nearest neighbor algorithm
    Neural networks (Multilayer perceptron)
    Similarity learning
*/

#[derive(Debug, Clone)]
pub enum SupervisedType {
    ElasticNet,
    DecisionTree {
        sub: decision_trees::DecisionTrees,
    },
    /// Support Vector Machines
    SVM {
        sub: svm::SupportVectorMachines,
    },
    Bayes,
    PLS,
}

impl Runner for SupervisedType {
    fn run(&self, cli: &crate::Cli) -> Result<(), Box<dyn Error>> {
        match self {
            SupervisedType::ElasticNet => todo!(),
            SupervisedType::DecisionTree { sub } => sub.run(cli),
            SupervisedType::SVM { sub } => sub.run(cli),
            SupervisedType::Bayes => todo!(),
            SupervisedType::PLS => todo!(),
        }
    }
}

impl TryFrom<&ArgMatches> for SupervisedType {
    type Error = String;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let Some((cmd_name, nxt_matches)) = matches.subcommand() else {
            //todo - throw exception
            panic!("todo-01 - can't find sub command");
        };

        match cmd_name {
            "ElasticNet" => Ok(SupervisedType::ElasticNet),
            decision_trees::CMD_DECISION_TREE => Ok(SupervisedType::DecisionTree {
                sub: decision_trees::DecisionTrees::try_from(nxt_matches)?,
            }),
            svm::CMD_SVM => Ok(SupervisedType::SVM {
                sub: svm::SupportVectorMachines::try_from(nxt_matches)?,
            }),
            "Bayes" => Ok(SupervisedType::Bayes),
            "PLS" => Ok(SupervisedType::PLS),

            _ => {
                error!("Algorithm \"{cmd_name}\" is not supported");
                std::process::exit(1);
            }
        }
    }
}

pub const CMD_SUPERVISED: &str = "SL";

pub fn command() -> Command {
    //add attributes

    let cmd_bayes = bayes::command();
    let cmd_decision_trees = decision_trees::command();
    let cmd_elastic_net = elastic_net::command();
    let cmd_pls = pls::command();
    let cmd_svm = svm::command();

    Command::new(CMD_SUPERVISED)
        .short_flag('c')
        .about("[s]upervised learning algorithms class")
        .subcommand_required(true)
        .subcommand(cmd_bayes)
        .subcommand(cmd_decision_trees)
        .subcommand(cmd_elastic_net)
        .subcommand(cmd_pls)
        .subcommand(cmd_svm)
}
