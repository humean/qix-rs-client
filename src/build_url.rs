use std::collections::HashMap;

use url::Url;

#[derive(Default, Debug)]
pub struct UrlBuilder {
    secure: bool,
    host: String,
    port: Option<u32>,
    prefix: Option<String>,
    subpath: Option<String>,
    route: Option<String>,
    identity: Option<String>,
    ttl: Option<String>,
    url_params: Vec<(String, String)>,
}

impl UrlBuilder {
    pub fn new() -> UrlBuilder {
        UrlBuilder {
            ..Default::default()
        }
    }

    pub fn create(self) -> String {
        let mut url = String::new();

        url.push_str(&self.host);

        url
    }

    pub fn with_hostname(&mut self, hostname: &str) {
        self.host = hostname.to_string();
    }

    pub fn is_secure(&mut self, secure: bool) {
        self.secure = secure;
    }

    pub fn with_port(&mut self, port: u32) {
        self.port = Some(port);
    }

    pub fn with_prefix(&mut self, _prefix: String) {
        unimplemented!();
    }

    pub fn with_subpath(&mut self, _subpath: String) {
        unimplemented!();
    }

    pub fn with_route(&mut self, _route: String) {
        unimplemented!();
    }

    pub fn with_ttl(&mut self, _ttl: String) {
        unimplemented!();
    }

    pub fn with_params(&mut self, _url_params: String) {
        unimplemented!();
    }
}
