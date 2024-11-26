use std::net::UdpSocket;

fn create_socket() -> std::io::Result<UdpSocket> {
    UdpSocket::bind("127.0.0.1:8080")
}

fn handle_socket(socket: UdpSocket) {
    let mut buf = [0; 4096];
    match socket.recv_from(&mut buf) {
        Ok((received, client_socket_addr)) => {
            println!("Received {} bytes", received);
            println!("Client: {:?}", client_socket_addr);
            println!(
                "User name: {:?}",
                String::from_utf8_lossy(&buf[..received]).trim()
            );
        }
        Err(e) => eprintln!("Failed to receive message: {}", e),
    }
}

fn start_server() {
    match create_socket() {
        Ok(socket) => handle_socket(socket),
        Err(e) => eprintln!("Failed to create socket: {}", e),
    }
}

fn main() {
    start_server();
}
