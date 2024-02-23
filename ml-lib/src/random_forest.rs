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
    let array = df.to_ndarray::<Float64Type>(IndexOrder::default())?;

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

    use linfa_trees::DecisionTree;
    let tree = DecisionTree::params().fit(&ds_transformed).unwrap();

    let accuracy = tree
        .predict(&ds_transformed)
        .confusion_matrix(&ds_transformed)
        .unwrap()
        .accuracy();

    println!("accuracy:{accuracy}");

    //b-close, b-volume, a-volume, a-close
    let test_pd = [
        5003.140137,
        5032.720215,
        4999.439941,
        5029.72998,
        4_137_970_000_f64,
        4_714_300_f64,
        186.869995,
    ];
    //2024-02-16
    let test_td = [
        5_031.129883,
        5_038.700195,
        4_999.52002,
        5_005.569824,
        3_833_270_000_f64,
        4_841_900_f64,
        187.639999,
    ];

    //2024-02-20
    let test_nd = [
        5_031.13, //b-open
        4_993.71, //b-h
        4_955.02, //b-l
        4_975.51, //b-c
        2_454_586_000_f64,
        4_242_877_f64,
        183.44,
    ];

    use ndarray::array;
    let records = array![test_pd, test_td, test_nd];
    let targets = array![0, 0, 0];

    let dataset = linfa::Dataset::new(records, targets).with_feature_names(feature_names);

    let zz = tree.predict(&dataset);
    println!("predicted:{zz}");

    Ok(())
}
