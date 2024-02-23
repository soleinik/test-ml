use clap::Command;

const CMD_PLS: &str = "PLS";

pub fn command() -> Command {
    Command::new(CMD_PLS)
        .short_flag('p')
        .about("[p]artial Least Squares")
        .long_about(
            "The PLS (Partial Least Squares) method is a statistical method that finds a linear\
        relationship between input variables and output variables by projecting them onto a new subspace\
        formed by newly chosen variables (aka latent variables), which are linear combinations\
        of the input variables. The subspace is choosen to maximize the covariance between\
        responses and independant variables.",
        )
}
