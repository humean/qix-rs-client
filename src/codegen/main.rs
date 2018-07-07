extern crate failure;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use failure::Error;
use serde_json::Value;
use std::fs::File;

struct QStructDef {
    name: String,
    kind: Value,
}

fn main() -> Result<(), Error> {
    let mut file = File::open("./jsonrpcapi.json").unwrap();

    let v: serde_json::Value = serde_json::from_reader(&file)?;
    let x = &v["definitions"];

    println!("{:?}", x);

    return Ok(());
}
