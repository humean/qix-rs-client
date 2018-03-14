extern crate qlik_rs;
extern crate ws;

fn main() {
    let a = qlik_rs::build_url::UrlBuilder::new();

    println!("{:?}", a);
}
