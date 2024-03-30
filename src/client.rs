use crate::setting::ClientSetting;
use crate::setting::APP_USER_AGENT;
use crate::ipinfo;

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
    blocking_inner: Option<reqwest::blocking::Client>,
    pub setting: ClientSetting,
}

impl Default for Client {
    fn default() -> Self {
        let setting = ClientSetting::default();
        Client {
            inner: reqwest::Client::builder().timeout(setting.timeout).build().unwrap(),
            blocking_inner: if setting.blocking {
                    Some(reqwest::blocking::Client::builder()
                    .timeout(setting.timeout)
                    .build().unwrap())
                } else {
                    None
                },
            setting: setting,
        }
    }
}

impl Client {
    pub fn new(setting: ClientSetting) -> Result<Self, Box<dyn std::error::Error>> {
        match &setting.proxy_scheme {
            Some(proxy_scheme) => {
                Ok(Client {
                    inner: reqwest::Client::builder()
                        .timeout(setting.timeout)
                        .proxy(reqwest::Proxy::all(proxy_scheme).unwrap())
                        .user_agent(setting.user_agent.as_deref().unwrap_or(APP_USER_AGENT))
                        .build()?,
                    blocking_inner: if setting.blocking {
                            Some(reqwest::blocking::Client::builder()
                            .timeout(setting.timeout)
                            .proxy(reqwest::Proxy::all(proxy_scheme).unwrap())
                            .user_agent(setting.user_agent.as_deref().unwrap_or(APP_USER_AGENT))
                            .build()?)
                        } else {
                            None
                        },
                    setting,
                })
            }
            None => {
                Ok(Client {
                    inner: reqwest::Client::builder()
                        .timeout(setting.timeout)
                        .build()?,
                    blocking_inner: if setting.blocking {
                            Some(reqwest::blocking::Client::builder()
                            .timeout(setting.timeout)
                            .build()?)
                        } else {
                            None
                        },
                    setting,
                })
            }
        }
    }
    pub async fn get_self_ip(&self) -> Result<ipinfo::IpInfoSimple, Box<dyn std::error::Error>> {
        let response = self.inner.get(ipinfo::BASE_URL).send().await?;
        Ok(response.json::<ipinfo::IpInfoSimple>().await?)
    }
    pub async fn get_self_ipv4(&self) -> Result<ipinfo::IpInfoSimple, Box<dyn std::error::Error>> {
        let response = self.inner.get(ipinfo::BASE_URL_V4).send().await?;
        Ok(response.json::<ipinfo::IpInfoSimple>().await?)
    }
    pub async fn get_self_ip_info(&self) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        let url = if self.setting.reverse_dns {
            format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP_REVERSE)
        } else {
            format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP)
        };
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::IpInfo>().await?)
    }
    pub async fn get_self_ipv4_info(&self) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        let url = if self.setting.reverse_dns {
            format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_IP_REVERSE)
        } else {
            format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_IP)
        };
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::IpInfo>().await?)
    }
    pub async fn get_header_info(&self) -> Result<ipinfo::HeaderInfo, Box<dyn std::error::Error>> {
        let url = format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_HEADER);
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::HeaderInfo>().await?)
    }
    pub async fn get_header_info_ipv4(&self) -> Result<ipinfo::HeaderInfo, Box<dyn std::error::Error>> {
        let url = format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_HEADER);
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::HeaderInfo>().await?)
    }
    pub async fn get_ip_info(&self, ip: &str) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        let url = if self.setting.reverse_dns {
            format!("{}{}/{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP_REVERSE, ip)
        } else {
            format!("{}{}/{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP, ip)
        };
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::IpInfo>().await?)
    }
    pub fn get_self_ip_blocking(&self) -> Result<ipinfo::IpInfoSimple, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let response = blocking_inner.get(ipinfo::BASE_URL).send()?;
                Ok(response.json::<ipinfo::IpInfoSimple>()?)
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
    pub fn get_self_ipv4_blocking(&self) -> Result<ipinfo::IpInfoSimple, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let response = blocking_inner.get(ipinfo::BASE_URL_V4).send()?;
                Ok(response.json::<ipinfo::IpInfoSimple>()?)
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
    pub fn get_self_ip_info_blocking(&self) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let url = if self.setting.reverse_dns {
                    format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP_REVERSE)
                } else {
                    format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP)
                };
                let response = blocking_inner.get(&url).send()?;
                Ok(response.json::<ipinfo::IpInfo>()?)
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
    pub fn get_self_ipv4_info_blocking(&self) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let url = if self.setting.reverse_dns {
                    format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_IP_REVERSE)
                } else {
                    format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_IP)
                };
                let response = blocking_inner.get(&url).send()?;
                Ok(response.json::<ipinfo::IpInfo>()?)
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
    pub fn get_header_info_blocking(&self) -> Result<ipinfo::HeaderInfo, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let url = format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_HEADER);
                let response = blocking_inner.get(&url).send()?;
                Ok(response.json::<ipinfo::HeaderInfo>()?)
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
    pub fn get_header_info_ipv4_blocking(&self) -> Result<ipinfo::HeaderInfo, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let url = format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_HEADER);
                let response = blocking_inner.get(&url).send()?;
                Ok(response.json::<ipinfo::HeaderInfo>()?)
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
    pub fn get_ip_info_blocking(&self, ip: &str) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let url = if self.setting.reverse_dns {
                    format!("{}{}/{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP_REVERSE, ip)
                } else {
                    format!("{}{}/{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP, ip)
                };
                let response = blocking_inner.get(&url).send()?;
                Ok(response.json::<ipinfo::IpInfo>()?)
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
}
