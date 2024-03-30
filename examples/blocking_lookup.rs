use std::net::IpAddr;

use ipstruct::client::Client;
use ipstruct::ipinfo::{HeaderInfo, IpInfo};
use ipstruct::setting::ClientSetting;

fn main() {
    println!(
        "Blocking IP info lookup {}",
        ipstruct::setting::DEFAULT_USER_AGENT
    );
    let setting: ClientSetting = ClientSetting::default().blocking(true);
    let client: Client = Client::new(setting).unwrap();
    let ip_addr: IpAddr = client.get_self_ip_blocking().unwrap();
    println!("{}", ip_addr);
    let ipv4_addr: IpAddr = client.get_self_ipv4_blocking().unwrap();
    println!("{:?}", ipv4_addr);
    let ip_info: IpInfo = client.get_self_ip_info_blocking().unwrap();
    println!("{:?}", ip_info);
    let ip_info: IpInfo = client.get_self_ipv4_info_blocking().unwrap();
    println!("{:?}", ip_info);
    let header_info: HeaderInfo = client.get_header_info_blocking().unwrap();
    println!("{:?}", header_info);
    let header_info: HeaderInfo = client.get_header_info_ipv4_blocking().unwrap();
    println!("{:?}", header_info);
    let ip_info: IpInfo = client.get_ip_info_blocking("1.1.1.1").unwrap();
    println!("{:?}", ip_info);
}
