use std::net::IpAddr;

use crate::setting::ClientSetting;
use crate::setting::DEFAULT_USER_AGENT;
use crate::ipinfo;

/// API client for ipstruct.com
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
    /// Create a new client with the given setting
    pub fn new(setting: ClientSetting) -> Result<Self, Box<dyn std::error::Error>> {
        match &setting.proxy_scheme {
            Some(proxy_scheme) => {
                Ok(Client {
                    inner: reqwest::Client::builder()
                        .timeout(setting.timeout)
                        .proxy(reqwest::Proxy::all(proxy_scheme).unwrap())
                        .user_agent(setting.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT))
                        .build()?,
                    blocking_inner: if setting.blocking {
                            Some(reqwest::blocking::Client::builder()
                            .timeout(setting.timeout)
                            .proxy(reqwest::Proxy::all(proxy_scheme).unwrap())
                            .user_agent(setting.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT))
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
                        .user_agent(setting.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT))
                        .build()?,
                    blocking_inner: if setting.blocking {
                            Some(reqwest::blocking::Client::builder()
                            .timeout(setting.timeout)
                            .user_agent(setting.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT))
                            .build()?)
                        } else {
                            None
                        },
                    setting,
                })
            }
        }
    }
    /// Get public IP address without any additional information.
    /// 
    /// Supports both IPv4 and IPv6.
    pub async fn get_self_ip(&self) -> Result<IpAddr, Box<dyn std::error::Error>> {
        let response = self.inner.get(ipinfo::BASE_URL).send().await?;
        match response.json::<ipinfo::IpInfoSimple>().await {
            Ok(ip_info) => {
                match ip_info.ip_addr.parse::<IpAddr>() {
                    Ok(ip_addr) => Ok(ip_addr),
                    Err(_) => Err("Failed to parse IP address".into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }
    /// Get public IPv4 address without any additional information.
    pub async fn get_self_ipv4(&self) -> Result<IpAddr, Box<dyn std::error::Error>> {
        let response = self.inner.get(ipinfo::BASE_URL_V4).send().await?;
        match response.json::<ipinfo::IpInfoSimple>().await {
            Ok(ip_info) => {
                match ip_info.ip_addr.parse::<IpAddr>() {
                    Ok(ip_addr) => Ok(ip_addr),
                    Err(_) => Err("Failed to parse IP address".into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }
    /// Get public IP address with additional information. 
    /// 
    /// Supports both IPv4 and IPv6.
    /// 
    /// This includes IP address, country, ASN, AS Name(ISP), and hostname(optional).
    pub async fn get_self_ip_info(&self) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        let url = if self.setting.reverse_dns {
            format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP_REVERSE)
        } else {
            format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP)
        };
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::IpInfo>().await?)
    }
    /// Get public IPv4 address with additional information.
    /// 
    /// This includes IP address, country, ASN, AS Name(ISP), and hostname(optional).
    pub async fn get_self_ipv4_info(&self) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        let url = if self.setting.reverse_dns {
            format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_IP_REVERSE)
        } else {
            format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_IP)
        };
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::IpInfo>().await?)
    }
    /// Get header information of the request.
    pub async fn get_header_info(&self) -> Result<ipinfo::HeaderInfo, Box<dyn std::error::Error>> {
        let url = format!("{}{}", ipinfo::BASE_URL, ipinfo::ROUTE_HEADER);
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::HeaderInfo>().await?)
    }
    /// Get header information of the request for IPv4.
    pub async fn get_header_info_ipv4(&self) -> Result<ipinfo::HeaderInfo, Box<dyn std::error::Error>> {
        let url = format!("{}{}", ipinfo::BASE_URL_V4, ipinfo::ROUTE_HEADER);
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::HeaderInfo>().await?)
    }
    /// Get IP information of the given IP address.
    pub async fn get_ip_info(&self, ip: &str) -> Result<ipinfo::IpInfo, Box<dyn std::error::Error>> {
        let url = if self.setting.reverse_dns {
            format!("{}{}/{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP_REVERSE, ip)
        } else {
            format!("{}{}/{}", ipinfo::BASE_URL, ipinfo::ROUTE_IP, ip)
        };
        let response = self.inner.get(&url).send().await?;
        Ok(response.json::<ipinfo::IpInfo>().await?)
    }
    /// Get public IP address without any additional information.
    pub fn get_self_ip_blocking(&self) -> Result<IpAddr, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let response = blocking_inner.get(ipinfo::BASE_URL).send()?;
                match response.json::<ipinfo::IpInfoSimple>() {
                    Ok(ip_info) => {
                        match ip_info.ip_addr.parse::<IpAddr>() {
                            Ok(ip_addr) => Ok(ip_addr),
                            Err(_) => Err("Failed to parse IP address".into()),
                        }
                    }
                    Err(e) => Err(e.into()),
                }
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
    /// Get public IPv4 address without any additional information.
    pub fn get_self_ipv4_blocking(&self) -> Result<IpAddr, Box<dyn std::error::Error>> {
        match &self.blocking_inner {
            Some(blocking_inner) => {
                let response = blocking_inner.get(ipinfo::BASE_URL_V4).send()?;
                match response.json::<ipinfo::IpInfoSimple>() {
                    Ok(ip_info) => {
                        match ip_info.ip_addr.parse::<IpAddr>() {
                            Ok(ip_addr) => Ok(ip_addr),
                            Err(_) => Err("Failed to parse IP address".into()),
                        }
                    }
                    Err(e) => Err(e.into()),
                }
            }
            None => {
                Err("Blocking is not enabled".into())
            }
        }
    }
    /// Get public IP address with additional information.
    /// 
    /// This includes IP address, country, ASN, AS Name(ISP), and hostname(optional).
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
    /// Get public IPv4 address with additional information.
    /// 
    /// This includes IP address, country, ASN, AS Name(ISP), and hostname(optional).
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
    /// Get header information of the request.
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
    /// Get header information of the request for IPv4.
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
    /// Get IP information of the given IP address.
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
