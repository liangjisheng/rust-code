// IpAddr实现了FromStr，可直接将代表IP地址的字符串解析为IpAddr

// SocketAddr代表包含了IP地址和端口号的套接字地址，它封装了ipv4套接字地址和ipv6套接字地址
// pub enum SocketAddr {
//     V4(SocketAddrV4),
//     V6(SocketAddrV6),
// }

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4};

fn main() {
    let localhost: IpAddr = "127.0.0.1".parse().unwrap();
    println!("localhost {}", localhost);

    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!("127.0.0.1".parse(), Ok(localhost));

    // is_ipv4()：是否是一个ipv4地址
    // is_ipv6()：是否是一个ipv6地址
    // is_loopback()：是否是一个 loopback 地址
    // is_multicast()：是否是一个多播地址
    // is_unspecified()：是否是一个 0.0.0.0 地址

    let addr = Ipv4Addr::new(127, 0, 0, 1);
    assert_eq!(addr.octets(), [127, 0, 0, 1]);

    let socket: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("socket {}", socket);

    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, 8080);
    println!("socket {}", socket);

    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    assert_eq!("127.0.0.1:8080".parse(), Ok(socket));
    assert_eq!(socket.ip(), &Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(socket.port(), 8080);
}
