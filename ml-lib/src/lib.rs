use std::fmt::Display;
use std::path::Path;

use linfa::metrics::ConfusionMatrix;
use log::debug;
use polars::frame::DataFrame;
use polars::lazy::dsl::when;
use polars::lazy::frame::LazyFrame;
use polars::lazy::{
    dsl::{col, lit},
    frame::{LazyCsvReader, LazyFileListReader},
};

mod algo_01;
pub mod random_forest;
pub mod svm;

pub mod preamble {
    pub use super::random_forest;
    pub use super::svm;
}

//Date,Open,High,Low,Close,Adj Close,Volume
fn get_df<P: AsRef<Path>>(name: &str, path: P) -> Result<LazyFrame, Box<dyn std::error::Error>> {
    let mut lazy_df = LazyCsvReader::new(path).has_header(true).finish()?;

    lazy_df = lazy_df.drop(["Close"]);
    lazy_df = lazy_df.rename(["Open"], [format!("{name}-open")]);
    lazy_df = lazy_df.rename(["High"], [format!("{name}-high")]);
    lazy_df = lazy_df.rename(["Low"], [format!("{name}-low")]);
    lazy_df = lazy_df.rename(["Adj Close"], [format!("{name}-close")]);
    lazy_df = lazy_df.rename(["Volume"], [format!("{name}-volume")]);

    Ok(lazy_df)
}

//pub(crate)
fn load<P: AsRef<Path>>(base: P, sup: P) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut a_df = get_df("A", base)?;
    a_df = a_df.drop(["A-open"]);
    a_df = a_df.drop(["A-high"]);
    a_df = a_df.drop(["A-low"]);

    let b_df = get_df("B", sup)?;

    a_df = a_df.with_column(col("A-close").alias("A-tomorrow").shift(lit(-1)));

    a_df = a_df.select(vec![
        //col("Date"), //index?
        col("A-volume"),
        col("A-close"),
        col("A-tomorrow"),
        when(col("A-tomorrow").gt(col("A-close")))
            .then(lit(1))
            .otherwise(lit(0))
            .alias("Predictions"),
    ]);

    let df = b_df.collect()?.hstack(a_df.collect()?.get_columns())?;

    let mut df = df.drop_nulls::<String>(None)?;

    df = df.drop("A-tomorrow")?;
    debug!("{df}");

    Ok(df)
}

fn print_cm_stats<T: Display>(cm: &ConfusionMatrix<T>) {
    println!("confusion matrix:{cm:?}");
    println!("accuracy {}", cm.accuracy(),);
    println!("precision {}", cm.precision(),);
    println!("recall {}", cm.recall(),);
}
