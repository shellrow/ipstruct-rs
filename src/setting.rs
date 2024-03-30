use std::time::Duration;

pub(crate) const DEFAULT_USER_AGENT_FIREFOX: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/121.0";
pub(crate) const DEFAULT_USER_AGENT_CHROME: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
/// Default User Agent
pub static DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"-rust-client","/",env!("CARGO_PKG_VERSION"));

/// Client Setting
#[derive(Debug, Clone)]
pub struct ClientSetting {
    /// Timeout for the request
    pub timeout: Duration,
    /// User Agent for the request
    pub user_agent: Option<String>,
    /// Proxy Scheme for the request
    pub proxy_scheme: Option<String>,
    /// Blocking or non-blocking
    pub blocking: bool,
    /// Reverse DNS lookup for the request
    pub reverse_dns: bool,
}

impl Default for ClientSetting {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientSetting {
    /// Create a new instance of ClientSetting
    pub fn new() -> Self {
        ClientSetting {
            timeout: Duration::from_secs(10),
            user_agent: None,
            proxy_scheme: None,
            blocking: false,
            reverse_dns: false,
        }
    }
    /// Set the timeout for the request
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// Set the user agent for the request
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }
    /// Set the proxy scheme for the request
    pub fn proxy_scheme(mut self, proxy_scheme: &str) -> Self {
        self.proxy_scheme = Some(proxy_scheme.to_string());
        self
    }
    /// Set the user agent to Firefox
    pub fn user_agent_firefox(mut self) -> Self {
        self.user_agent = Some(DEFAULT_USER_AGENT_FIREFOX.to_string());
        self
    }
    /// Set the user agent to Chrome
    pub fn user_agent_chrome(mut self) -> Self {
        self.user_agent = Some(DEFAULT_USER_AGENT_CHROME.to_string());
        self
    }
    /// Set blocking or non-blocking
    pub fn blocking(mut self, blocking: bool) -> Self {
        self.blocking = blocking;
        self
    }
    /// Set reverse DNS
    pub fn reverse_dns(mut self, reverse_dns: bool) -> Self {
        self.reverse_dns = reverse_dns;
        self
    }
    /// Set the user agent for the request
    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.user_agent = Some(user_agent.to_string());
    }
    /// Set the proxy scheme for the request
    pub fn set_proxy_scheme(&mut self, proxy_scheme: &str) {
        self.proxy_scheme = Some(proxy_scheme.to_string());
    }
    /// Set the timeout for the request
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
    /// Set reverse DNS
    pub fn set_reverse_dns(&mut self, reverse_dns: bool) {
        self.reverse_dns = reverse_dns;
    }
    /// Get the user agent for the request
    pub fn get_user_agent(&self) -> String {
        match &self.user_agent {
            Some(user_agent) => user_agent.to_string(),
            None => "".to_string(),
        }
    }
    /// Get the proxy scheme for the request
    pub fn get_proxy_scheme(&self) -> String {
        match &self.proxy_scheme {
            Some(proxy_scheme) => proxy_scheme.to_string(),
            None => "".to_string(),
        }
    }
    /// Get the timeout for the request
    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }
}
