use clap::Command;

pub const CMD_RANDOM_FOREST: &str = "RF";

pub fn command() -> Command {
    Command::new(CMD_RANDOM_FOREST)
        .short_flag('r')
        .about("[r]andom forest learning algorithm")
        .long_about(
            "Random forests or random decision forests is an ensemble learning method for classification,\
             regression and other tasks that operates by constructing a multitude of decision trees at training time.\
             For classification tasks, the output of the random forest is the class selected by most trees.\
             For regression tasks, the mean or average prediction of the individual trees is returned.\
             Random decision forests correct for decision trees' habit of overfitting to their training set.",
        )
}
