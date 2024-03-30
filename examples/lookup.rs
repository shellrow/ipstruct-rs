use ipstruct::client::Client;
use ipstruct::setting::ClientSetting;
use ipstruct::ipinfo::{IpInfoSimple, IpInfo, HeaderInfo};

#[tokio::main]
async fn main() {
    let setting: ClientSetting = ClientSetting::default();
    let client: Client = Client::new(setting).unwrap();
    let ip_info_simple: IpInfoSimple = client.get_self_ip().await.unwrap();
    println!("{:?}", ip_info_simple);
    let ip_info_simple = client.get_self_ipv4().await.unwrap();
    println!("{:?}", ip_info_simple);
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
