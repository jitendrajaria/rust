#[derive(Debug)]
struct IPAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
fn main() {
    let four = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;
    let home = IPAddr {
        address: String::from("0.0.0.0"),
        kind: IpAddrKind::V4,
    };
    println!("Home info is {:?}", home);
    route(four);
}

fn route(ip_addr: IpAddrKind) {
    // println!("IP address is {:?}", ip_addr);
}
