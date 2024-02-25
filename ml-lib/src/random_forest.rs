use std::{error::Error, path::Path};

use linfa::prelude::*;
use linfa::traits::Fit;
use linfa_preprocessing::linear_scaling::LinearScaler;
use log::trace;
use ndarray::{s, Ix1};
use polars::{datatypes::Float64Type, frame::DataFrame, prelude::IndexOrder};

pub fn doit<P: AsRef<Path>>(base: P, sup: P) -> Result<(), Box<dyn std::error::Error>> {
    run(&crate::load(base, sup)?)
}

fn run(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    let array = df.to_ndarray::<Float64Type>(IndexOrder::C)?;

    let last_col = array.ncols() - 1;

    let (data, targets) = (
        array.slice(s![.., 1..last_col]).to_owned(), //skip date
        array.column(last_col).to_owned(),
    );
    //println!("{data}\n{targets}");

    let feature_names = vec![
        "B-open", "B-high", "B-low", "B-close", "B-volume", "A-volume", "A-close",
    ];

    //println!("target:{targets}");
    let dataset = linfa::Dataset::new(data, targets)
        .map_targets(|x| *x as usize)
        .with_feature_names(feature_names.clone());

    let model = linfa::traits::Fit::fit(&LinearScaler::<f64>::min_max(), &dataset)?;
    let ds_transformed: Dataset<f64, usize, Ix1> = model.transform(dataset); //: Dataset<f64, usize, Ix1>

    //normalized
    trace!("{ds_transformed:#?}");

    let (train, test) = ds_transformed.split_with_ratio(0.9);

    use linfa_trees::DecisionTree;
    let model = DecisionTree::params().fit(&train).unwrap();

    let predict = model.predict(&test);
    let cm = predict.confusion_matrix(&test)?;
    println!("confusion matrix:{cm:?}");
    let accuracy = cm.accuracy();
    println!("accuracy:{accuracy}");

    Ok(())
}
