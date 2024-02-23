use std::error::Error;

use ndarray::s;
use polars::{datatypes::Float32Type, frame::DataFrame, prelude::IndexOrder};

pub fn run(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    //println!("{df}");
    let array = df.to_ndarray::<Float32Type>(IndexOrder::default()).unwrap();

    let last_col = array.ncols() - 1;

    let (data, targets) = (
        array.slice(s![.., 1..last_col]).to_owned(), //skip date
        array.column(last_col).to_owned(),
    );
    //println!("{data}\n{targets}");

    let num_samples = data.nrows();
    let feature_names = vec![
        "B-open".to_string(),
        "B-high".to_string(),
        "B-low".to_string(),
        "B-close".to_string(),
        "B-volume".to_string(),
        "A-volume".to_string(),
        "A-close".to_string(),
    ];

    let ds = smartcore::dataset::Dataset {
        data: data.into_raw_vec(),
        target: targets.into_raw_vec(),
        num_samples: num_samples,
        num_features: feature_names.len(),
        feature_names: feature_names,
        target_names: vec!["UpOrDown[0, 1]".to_string()],
        description: "test".to_string(),
    };

    let settings = automl::Settings::default_classification()
        // .with_random_forest_classifier_settings(
        //     RandomForestClassifierParameters::default()
        //         .with_min_samples_split(100)
        //         .with_seed(1),
        // )
        .only(automl::settings::Algorithm::RandomForestClassifier);

    // let settings = automl::Settings::default_classification()
    // .only(automl::settings::Algorithm::DecisionTreeClassifier);

    // let settings =
    //     automl::Settings::default_classification().only(automl::settings::Algorithm::KNNClassifier);
    // let settings = automl::Settings::default_classification()
    //     .only(automl::settings::Algorithm::GaussianNaiveBayes);

    let mut classifier = automl::SupervisedModel::new(ds, settings);
    classifier.train();

    println!("{classifier}");

    //b-close, b-volume, a-volume, a-close
    let test_pd = vec![
        5003.140137,
        5032.720215,
        4999.439941,
        5029.72998,
        4137970000f32,
        4714300f32,
        186.869995,
    ];
    let test_td = vec![
        5031.129883,
        5038.700195,
        4999.52002,
        5005.569824,
        3833270000f32,
        4841900f32,
        187.639999,
    ];

    let test_nd = vec![
        5_031.13,
        4_993.71,
        4_955.02,
        4_975.51,
        2_454_586_000f32,
        4_242_877f32,
        183.44,
    ];

    let res = classifier.predict(vec![test_pd, test_td, test_nd]);
    println!("{res:#?}");

    Ok(())
}
