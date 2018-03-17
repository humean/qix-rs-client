use std::borrow::Borrow;
use url::Url;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use failure::Error;
use regex::Regex;

/// Builds a url string for connecting to a Qlik Server Engine APIs. Internally
/// uses the url crate to validate that the url is well formed.
///
/// Example:
///
/// ```rust
///
/// let url = UrlBuilder::new()
///     .with_hostname("localhost")
///     .with_secure(true)
///     .with_port(4848)
///     .build()
///     .unwrap(); // "wss://localhost:4848"
///
/// ```
#[derive(Default, Debug, Clone)]
pub struct UrlBuilder {
    secure: bool,
    host: String,
    port: Option<u32>,
    prefix: Option<String>,
    subpath: Option<String>,
    route: Option<String>,
    app_id: Option<String>,
    identity: Option<String>,
    ttl: Option<String>,
    url_params: Vec<(String, String)>,
}

impl UrlBuilder {
    /// Create new UrlBuilder
    pub fn new() -> UrlBuilder {
        UrlBuilder {
            ..Default::default()
        }
    }

    /// Consumes UrlBuilder and creates url string from builder input. Returns
    /// a Result as it gets validated by `url::prase` from url crate
    pub fn build(self) -> Result<String, Error> {
        let mut url = String::new();

        match self.secure {
            true => url.push_str("wss://"),
            false => url.push_str("ws://"),
        }

        if self.host == "" {
            url.push_str("localhost");
        } else {
            url.push_str(&self.host);
        }

        match self.port {
            Some(port) => url.push_str(&format!(":{}", port.to_string())),
            None => {}
        }

        match self.prefix {
            Some(ref prefix) => url.push_str(&format!("/{}", prefix)),
            None => {}
        };

        match self.subpath {
            Some(ref prefix) => url.push_str(&format!("/{}", prefix)),
            None => {}
        }

        match self.route {
            Some(ref route) => url.push_str(&format!("/{}", route)),
            None => match self.app_id {
                Some(ref app_id) => url.push_str(&format!("/app/{}", app_id)),
                None => {}
            },
        }

        match self.identity {
            Some(ref identity) => url.push_str(&format!("/identity/{}", identity)),
            None => {}
        }

        match self.ttl {
            Some(ref ttl) => url.push_str(&format!("/ttl/{}", ttl)),
            None => {}
        }

        let url = Url::parse(&url)?;

        Ok(url.into_string())
    }

    /// Sets the hostname
    pub fn with_hostname(mut self, hostname: &str) -> Self {
        self.host = strip_leading_trailing_slashes(hostname);
        self
    }

    /// Sets whether to us `ws://` or `wss://`
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Sets the port
    pub fn with_port(mut self, port: u32) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_app_id(mut self, app_id: &str) -> Self {
        self.app_id = Some(utf8_percent_encode(app_id, DEFAULT_ENCODE_SET).to_string());
        self
    }

    /// Sets a Qlik proxy prefix
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(strip_leading_trailing_slashes(prefix));
        self
    }

    pub fn with_identity(mut self, identity: &str) -> Self {
        self.identity = Some(utf8_percent_encode(identity, DEFAULT_ENCODE_SET).to_string());
        self
    }

    pub fn with_subpath(mut self, _subpath: &str) -> Self {
        unimplemented!();
    }

    pub fn with_route(mut self, _route: &str) -> Self {
        unimplemented!();
    }

    pub fn with_ttl(mut self, _ttl: &str) -> Self {
        unimplemented!();
    }

    pub fn with_params<T, K, V>(mut self, url_params: T) -> Self
    where
        T: IntoIterator,
        T::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        unimplemented!();
    }
}

/// Stripping paths of all leading and trailing forward slashes "/"
fn strip_leading_trailing_slashes(s: &str) -> String {
    let re = Regex::new("(^[/]+)|([/]+$)").unwrap();

    re.replace_all(s, "").into_owned()
}
