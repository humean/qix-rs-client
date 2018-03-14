extern crate qlik_rs;
extern crate ws;

use ws::{connect, CloseCode};

fn main() {
    let a = qlik_rs::build_url::UrlBuilder::new();

    println!("{:?}", a);
}
