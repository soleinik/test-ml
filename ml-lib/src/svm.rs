use std::{error::Error, path::Path};

use linfa::prelude::ToConfusionMatrix;
use linfa::{
    dataset::Pr,
    traits::{Fit, Predict},
    MultiClassModel,
};
use linfa_svm::Svm;
use ndarray::s;
use polars::{datatypes::Float64Type, frame::DataFrame, prelude::IndexOrder};

pub fn doit<P: AsRef<Path>>(base: P, sup: P, eps: f64) -> Result<(), Box<dyn std::error::Error>> {
    run(&crate::load(base, sup)?, eps)
}

fn run(df: &DataFrame, eps: f64) -> Result<(), Box<dyn Error>> {
    let array = df.to_ndarray::<Float64Type>(IndexOrder::C)?;

    let last_col = array.ncols() - 1;

    let (data, targets) = (
        array.slice(s![.., 1..last_col]).to_owned(), //skip date
        array.column(last_col).to_owned(),
    );

    let feature_names = vec![
        "B-open", "B-high", "B-low", "B-close", "B-volume", "A-volume", "A-close",
    ];

    //println!("target:{targets}");
    let dataset = linfa::Dataset::new(data, targets)
        .map_targets(|x| *x as usize)
        .with_feature_names(feature_names.clone());

    let (train, test) = dataset.split_with_ratio(0.9);

    let params = Svm::<_, Pr>::params().gaussian_kernel(eps);

    let model = train
        .one_vs_all()?
        .into_iter()
        .filter(|(k, _x)| *k == 1)
        .map(|(k, x)| (k, params.fit(&x).unwrap()))
        .collect::<MultiClassModel<_, _>>();

    let predict = model.predict(&test);
    let cm = predict.confusion_matrix(&test)?;
    crate::print_cm_stats(&cm);

    Ok(())
}
