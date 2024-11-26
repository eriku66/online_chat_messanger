use std::net::UdpSocket;

fn main() {
    println!("Please enter your name:");

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket
        .send_to(&name.bytes().collect::<Vec<_>>(), "127.0.0.1:8080")
        .unwrap();
}
