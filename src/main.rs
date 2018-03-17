extern crate qlik_rs;
extern crate serde;
extern crate serde_json;
extern crate ws;

use std::process;
use qlik_rs::UrlBuilder;

fn main() {
    let a = UrlBuilder::new()
        .with_hostname("localhost")
        .with_prefix("mobile")
        .with_secure(true)
        .with_port(4848)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Problem building qlik server url: {}", err.cause());
            process::exit(1)
        });

    println!("{}", a);
}
