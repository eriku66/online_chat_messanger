use std::net::UdpSocket;

fn create_socket() -> std::io::Result<UdpSocket> {
    UdpSocket::bind(shared::SERVER_ADDR)
}

fn handle_socket(socket: UdpSocket) {
    let mut buf = [0; shared::MAX_MESSAGE_SIZE_BYTES];
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
