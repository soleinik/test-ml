use clap::Command;

const CMD_SVM: &str = "SVM";

pub fn command() -> Command {
    Command::new(CMD_SVM)
        .short_flag('s')
        .about("[s]upport Vector Machines")
        .long_about("Support Vector Machines are a major branch of machine learning models and offer\
         classification or regression analysis of labeled datasets. They seek a discriminant, which\
         separates the data in an optimal way, e.g. have the fewest numbers of miss-classifications and\
         maximizes the margin between positive and negative classes. A support vector contributes to the\
         discriminant and is therefore important for the classification/regression task. The balance\
         between the number of support vectors and model performance can be controlled with hyperparameters.")
}
