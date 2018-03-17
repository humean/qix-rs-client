use url::Url;
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
    /// Create new UrlBuilder
    pub fn new() -> UrlBuilder {
        UrlBuilder {
            ..Default::default()
        }
    }

    /// Creates url string from builder input
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

        let url = Url::parse(&url)?;

        Ok(url.into_string())
    }

    /// Sets the hostname
    pub fn with_hostname(&mut self, hostname: &str) -> &mut Self {
        self.host = strip_leading_trailing_slashes(hostname);
        self
    }

    /// Sets whether to us `ws://` or `wss://`
    pub fn with_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;
        self
    }

    /// Sets the port
    pub fn with_port(&mut self, port: u32) -> &mut Self {
        self.port = Some(port);
        self
    }

    /// Sets a Qlik proxy prefix
    pub fn with_prefix(&mut self, prefix: &str) -> &mut Self {
        self.prefix = Some(strip_leading_trailing_slashes(prefix));
        self
    }

    pub fn with_subpath(&mut self, _subpath: &str) -> &mut Self {
        unimplemented!();
    }

    pub fn with_route(&mut self, _route: &str) -> &mut Self {
        unimplemented!();
    }

    pub fn with_ttl(&mut self, _ttl: &str) -> &mut Self {
        unimplemented!();
    }

    pub fn with_params(&mut self, _url_params: &str) -> &mut Self {
        unimplemented!();
    }
}

/// Stripping paths of all leading and trailing forward slashes "/"
fn strip_leading_trailing_slashes(s: &str) -> String {
    let re = Regex::new("(^[/]+)|([/]+$)").unwrap();

    re.replace_all(s, "").into_owned()
}
