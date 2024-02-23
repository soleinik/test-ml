use clap::Command;

const CMD_ELASTIC_NET: &str = "EN";

pub fn command() -> Command {
    Command::new(CMD_ELASTIC_NET)
        .short_flag('e')
        .about("[e]lastic net linear regression")
        .long_about(
            "an elastic net implementation for linear regression models. It combines\
         l1 and l2 penalties of the LASSO and ridge methods and offers therefore a greater\
         flexibility for feature selection. With increasing penalization certain parameters\
         become zero, their corresponding variables are dropped from the model.",
        )
}
