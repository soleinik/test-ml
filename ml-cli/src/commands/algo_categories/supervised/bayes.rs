use clap::Command;

const CMD_BAYES: &str = "bayes";
pub fn command() -> Command {
    Command::new(CMD_BAYES)
        .short_flag('b')
        .about("[b]ayesian Inference")
        .long_about("Bayesian inference is a specific way to learn from data that is heavily used\
         in statistics for data analysis. Bayesian inference is used less often in the field of machine\
         learning, but it offers an elegant framework to understand what “learning” actually is.")
}
