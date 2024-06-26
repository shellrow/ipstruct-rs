pub const BASE_URL: &str = "https://api.ipstruct.com/";
pub const BASE_URL_V4: &str = "https://ipv4.ipstruct.com/";
pub(crate) const ROUTE_IP: &str = "ip";
pub(crate) const ROUTE_HEADER: &str = "header";
pub(crate) const ROUTE_IP_REVERSE: &str = "ip/reverse";

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct IpInfoSimple {
    pub ip_addr: String,
    pub api_version: String,
    pub message: String,
}

/// IP Address information
/// This includes IP address, country, ASN, AS Name(ISP), and hostname(optional).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IpInfo {
    pub ip_version: String,
    pub ip_addr_dec: String,
    pub ip_addr: String,
    pub host_name: String,
    pub network: String,
    pub asn: String,
    pub as_name: String,
    pub country_code: String,
    pub country_name: String,
}

/// Request header information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderInfo {
    pub ip_addr: String,
    pub accept: String,
    pub accept_encoding: String,
    pub accept_language: String,
    pub cache_control: String,
    pub connection: String,
    pub dnt: String,
    pub schema: String,
    pub authority: String,
    pub path: String,
    pub request_method: String,
    pub user_agent: String,
    pub referer: String,
}
