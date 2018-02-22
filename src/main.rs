extern crate qlik_rs;

extern crate failure;
extern crate serde;
extern crate serde_json;

use failure::Error;
use serde_json::Value;

use qlik_rs::QHyperCubeDef;

fn main() {
    let a = QHyperCubeDef::new();

    let b = serde_json::to_string(&a).unwrap();

    println!("{}", b);
}
