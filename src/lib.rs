//! # Qlik-rs
//! Qlik-rs is a implemention of the Qlik Engine APIs in rust for use by either
//! a server or web client.
//!

extern crate failure;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate url;

mod build_url;
pub mod hypercube_def;

pub use build_url::UrlBuilder;