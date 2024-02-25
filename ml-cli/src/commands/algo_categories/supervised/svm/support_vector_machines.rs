use clap::{value_parser, Arg, Command};

pub const CMD_SUPPORT_VECTOR_MACHINES: &str = "SVM";
pub const ARG_EPS: &str = "eps";
pub const DEFAULT_EPS: &str = "30.0";

pub fn command() -> Command {
    let arg_eps = Arg::new(ARG_EPS)
        .short('e')
        .long("eps")
        .required(false)
        .help("eps distribution")
        .long_help("Sets the model to use the Gaussian kernel")
        .default_value(DEFAULT_EPS)
        .value_parser(value_parser!(f64));

    Command::new(CMD_SUPPORT_VECTOR_MACHINES)
        .short_flag('s')
        .about("[s]upport vector machines learning algorithm")
        .long_about(
            "SVMs are a type of supervised learning algorithm that can be used for regression or classification.\
             They are often used for forecasting financial markets because they can handle nonlinear relationships\
             between variables. Their strengths include high accuracy, robustness, and the ability to handle large\
             datasets. However, their main drawback is that they can be sensitive to the choice of kernel function\
             and parameter tuning.",
        )
        .arg(arg_eps)
}
