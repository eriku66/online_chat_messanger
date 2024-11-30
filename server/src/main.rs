use anyhow::{Context, Result};
use std::net::UdpSocket;

fn create_socket() -> std::io::Result<UdpSocket> {
    UdpSocket::bind(shared::SERVER_ADDR)
}

fn handle_socket(socket: UdpSocket) -> Result<()> {
    let mut buf = [0; shared::MAX_MESSAGE_SIZE_BYTES];
    let (received, client_socket_addr) = socket
        .recv_from(&mut buf)
        .context("Failed to receive message")?;
    println!("Received {} bytes", received);
    println!("Client: {:?}", client_socket_addr);
    println!(
        "User name: {:?}",
        String::from_utf8_lossy(&buf[..received]).trim()
    );

    Ok(())
}

fn start_server() -> Result<()> {
    let socket = create_socket()?;
    handle_socket(socket)?;

    Ok(())
}

fn main() -> Result<()> {
    start_server()?;

    Ok(())
}
