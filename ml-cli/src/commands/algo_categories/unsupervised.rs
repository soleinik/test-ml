use std::error::Error;

use clap::{ArgMatches, Command};

use crate::Runner;

#[derive(Debug, Clone)]
pub enum UnsupervisedType {
    Clustering,
    Hierarchical,
    ICA,
    TSNE,
}

impl Runner for UnsupervisedType {
    fn run(&self, cli: &crate::Cli) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl TryFrom<&ArgMatches> for UnsupervisedType {
    type Error = String;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let Some((cmd_name, _matches)) = matches.subcommand() else {
            //todo - throw exception
            panic!("todo-01 - can't find sub command");
        };
        todo!()
        // match cmd_name {
        //     "ElasticNet" => Ok(Supervised::ElasticNet),
        //     "Trees" => Ok(Supervised::Trees),
        //     "SVM" => Ok(Supervised::SVM),
        //     "Bayes" => Ok(Supervised::Bayes),
        //     "PLS" => Ok(Supervised::PLS),

        //     _ => {
        //         error!("Algorithm \"{cmd_name}\" is not supported");
        //         std::process::exit(1);
        //     }
        // }
    }
}

pub const CMD_UNSUPERVISED: &str = "USL";
pub fn command() -> Command {
    Command::new(CMD_UNSUPERVISED)
        .short_flag('u')
        .about("[u]nSupervised learning algorithm class")
}
