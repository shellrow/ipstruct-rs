use std::net::IpAddr;

use ipstruct::client::Client;
use ipstruct::ipinfo::{HeaderInfo, IpInfo};
use ipstruct::setting::ClientSetting;

#[tokio::main]
async fn main() {
    println!("IP info lookup {}", ipstruct::setting::DEFAULT_USER_AGENT);
    let setting: ClientSetting = ClientSetting::default();
    let client: Client = Client::new(setting).unwrap();
    let ip_addr: IpAddr = client.get_self_ip().await.unwrap();
    println!("{}", ip_addr);
    let ipv4_addr: IpAddr = client.get_self_ipv4().await.unwrap();
    println!("{:?}", ipv4_addr);
    let ip_info: IpInfo = client.get_self_ip_info().await.unwrap();
    println!("{:?}", ip_info);
    let ip_info: IpInfo = client.get_self_ipv4_info().await.unwrap();
    println!("{:?}", ip_info);
    let header_info: HeaderInfo = client.get_header_info().await.unwrap();
    println!("{:?}", header_info);
    let header_info: HeaderInfo = client.get_header_info_ipv4().await.unwrap();
    println!("{:?}", header_info);
    let ip_info: IpInfo = client.get_ip_info("1.1.1.1").await.unwrap();
    println!("{:?}", ip_info);
}
