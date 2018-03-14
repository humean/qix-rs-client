//! # Qlik-rs
//! Qlik-rs is a implemention of the Qlik Engine APIs in rust for use by either
//! a server or web client.
//!

#[macro_use]
extern crate serde_derive;
extern crate url;

mod build_url;
mod hypercube_def;

pub use build_url::UrlBuilder;
pub use hypercube_def::*;
