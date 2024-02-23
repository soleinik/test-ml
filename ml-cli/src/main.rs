use clap::ArgMatches;
use commands::algo_categories::AlgoCategory;
use env_logger::Builder;
use log::LevelFilter;
use std::{error::Error, io::Write};

use crate::commands::algo_categories;

mod commands;

pub trait Runner {
    fn run(&self, cli: &Cli) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Cli {
    algo_category: AlgoCategory,
}

impl Runner for Cli {
    fn run(&self, cli: &Cli) -> Result<(), Box<dyn Error>> {
        self.algo_category.run(cli)
    }
}

impl TryFrom<ArgMatches> for Cli {
    type Error = String;

    fn try_from(matches: ArgMatches) -> Result<Self, Self::Error> {
        let Some((cmd_name, matches)) = matches.subcommand() else {
            //todo - throw exception
            panic!("todo-01 - can't find sub command");
        };
        debug_assert!(cmd_name == algo_categories::CMD_ALGO_CATEGORY);
        Ok(Cli {
            algo_category: AlgoCategory::try_from(matches).unwrap(),
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = commands::command().get_matches();
    let verbosity = matches.get_count("verbosity");

    //logger setup
    let mut builder = Builder::new();

    builder.format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()));

    match verbosity {
        0 => builder.filter(None, LevelFilter::Warn),
        1 => {
            println!("setting log to INFO level..");
            builder.filter(None, LevelFilter::Info)
        }
        2 => {
            println!("setting log to DEBUG level..");
            builder.filter(None, LevelFilter::Debug)
        }
        _ => {
            println!("setting log to TRACE level..");
            builder.filter(None, LevelFilter::Trace)
        }
    };

    builder.init();
    //logger setup end

    let cli = Cli::try_from(matches)?;
    cli.run(&cli)
}
