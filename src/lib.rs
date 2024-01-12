#![deny(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]
//! this is

use std::path::Path;

use linfa::prelude::*;
use linfa::traits::Fit;
use linfa_preprocessing::linear_scaling::LinearScaler;
use ndarray_csv::Array2Reader;
use polars::prelude::*;
// use polars::{
//     error::PolarsResult,
//     frame::DataFrame,
//     io::{csv::CsvReader, SerReader},
// };

//use std::fs::File;
use csv::ReaderBuilder;
use ndarray::Array2;

use ndarray::{prelude::*, OwnedRepr};

use textplots::{Chart, Plot, Shape};
// #[derive(Debug, serde::Deserialize)]
// struct Record(
//     //Date,Open,High,Low,Close,Adj Close,Volume
//     NaiveDate,
//     f32,
//     f32,
//     f32,
//     f32,
//     f32,
//     i64,
// );

// ///
// fn data<P: AsRef<Path>>(path: P) -> PolarsResult<DataFrame> {
//     CsvReader::from_path(path.as_ref())?
//         .has_header(true)
//         .finish()
// }

///
pub fn doit<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        //.delimiter(b',')
        .from_path(path)?;

    let array = reader.deserialize_array2_dynamic::<f64>()?;

    println!("{:?}", array);

    let scaler = LinearScaler::<f64>::min_max();

    let (data, targets) = (
        array.slice(s![.., 0..2]).to_owned(),
        array.column(2).to_owned(),
    );

    let feature_names = vec!["test 1", "test 2"];

    let dataset = linfa::dataset::Dataset::new(data, targets)
        .map_targets(|x| {
            if *x as usize == 1 {
                "accepted"
            } else {
                "denied"
            }
        })
        .with_feature_names(feature_names);

    //let df = data(path)?;

    //println!("data set: r:{} c:{}", df.height(), df.width());

    let zz = scaler.fit(&dataset).unwrap().transform(dataset.view());

    println!("{:?}", zz);

    // // for res in rdr.deserialize::<Record>() {
    // //     let r = res?;
    // //     println!("{r:?}");
    // // }

    // println!("y = sin(x) / x");

    // Chart::new(180, 60, 0.0, df.height())
    //     .lineplot(&Shape::Continuous(Box::new(|x| x.sin() / x)))
    //     .display();

    Ok(())
}
