use clap::Command;

pub const CMD_SUPPORT_VECTOR_MACHINES: &str = "SVM";

pub fn command() -> Command {
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
}
