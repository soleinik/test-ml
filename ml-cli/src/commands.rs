use clap::{Arg, ArgAction, Command};

pub mod algo_categories;

pub fn command() -> Command {
    let arg_verbosity = Arg::new("verbosity")
        .short('v')
        .long("verbosity")
        .action(ArgAction::Count)
        .required(false)
        .help("log level")
        .long_help("sets log level verbosity, default - WARN");

    // algo category -[1:n]->  algo group -[1:n] - algo name

    let cmd_algo_category = algo_categories::command();

    Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cmd_algo_category)
        .arg(arg_verbosity)
}
