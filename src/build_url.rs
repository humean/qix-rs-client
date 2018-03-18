use std::borrow::Borrow;
use url::Url;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use failure::Error;
use regex::Regex;

/// Builds a url string for connecting to a Qlik Sense Server Engine APIs via WebSockets.
/// Internally uses the url crate to validate that the url is well formed.
///
/// Example:
///
/// ```rust
/// use qlik_rs::UrlBuilder;
///
/// let url = UrlBuilder::new()
///     .with_hostname("localhost")
///     .with_secure(true)
///     .with_port(4848)
///     .build()
///     .unwrap();
///
///  assert!(url == "wss://localhost:4848/")
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
    ttl: Option<u32>,
    url_params: Vec<(String, String)>,
}

impl UrlBuilder {
    /// Create new UrlBuilder
    pub fn new() -> UrlBuilder {
        UrlBuilder {
            ..Default::default()
        }
    }

    /// Creates url string from builder. Returns a `Result` as it gets
    /// validated by `url::prase` from `url` crate
    pub fn build(&self) -> Result<String, Error> {
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

    /// Sets whether to us secure WebSockets. Default false
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Sets the port, with the default being the default WebSocket ports
    pub fn with_port(mut self, port: u32) -> Self {
        self.port = Some(port);
        self
    }

    /// Sets the ID of the app intended to be opened in the session
    pub fn with_app_id(mut self, app_id: &str) -> Self {
        self.app_id = Some(utf8_percent_encode(app_id, DEFAULT_ENCODE_SET).to_string());
        self
    }

    /// Sets a base absolute path when connecting, to be used with Qlik proxy prefix
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(strip_leading_trailing_slashes(prefix));
        self
    }

    /// Set an identity (session ID) to use
    pub fn with_identity(mut self, identity: &str) -> Self {
        self.identity = Some(utf8_percent_encode(identity, DEFAULT_ENCODE_SET).to_string());
        self
    }

    /// Sets the subpath to use. Used to connect to dataprepservice in a server environment
    pub fn with_subpath(mut self, subpath: &str) -> Self {
        self.subpath = Some(strip_leading_trailing_slashes(subpath));
        self
    }

    /// Sets initial route to open the WebSocket against
    pub fn with_route(mut self, route: &str) -> Self {
        self.route = Some(strip_leading_trailing_slashes(route));
        self
    }

    /// A value in seconds that QIX Engine should keep the session alive after
    /// socket disconnect (only works if QIX Engine supports it)
    pub fn with_ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Sets additional parameters to be added to WebSocket URL
    pub fn with_params<T, K, V>(self, _url_params: T) -> Self
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
