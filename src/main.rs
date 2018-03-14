extern crate qlik_rs;
extern crate serde;
extern crate serde_json;
extern crate ws;

use qlik_rs::{QHyperCubeDef, UrlBuilder};

fn main() {
    let a = UrlBuilder::new();

    let b = QHyperCubeDef::new();

    println!("{:?}", a);
    println!("{:?}", serde_json::to_string(&b).unwrap());
}
