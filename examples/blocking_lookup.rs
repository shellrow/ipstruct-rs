use ipstruct::client::Client;
use ipstruct::setting::ClientSetting;
use ipstruct::ipinfo::{IpInfoSimple, IpInfo, HeaderInfo};

fn main() {
    let setting: ClientSetting = ClientSetting::default().blocking(true);
    let client: Client = Client::new(setting).unwrap();
    let ip_info_simple: IpInfoSimple = client.get_self_ip_blocking().unwrap();
    println!("{:?}", ip_info_simple);
    let ip_info_simple = client.get_self_ipv4_blocking().unwrap();
    println!("{:?}", ip_info_simple);
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
