use anyhow::Result;
use std::net::UdpSocket;

#[derive(Debug)]
pub struct ClientSocket {
    pub socket: std::net::UdpSocket,
}

impl ClientSocket {
    pub fn new() -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind("127.0.0.1:0").unwrap(),
        })
    }

    pub fn send_to(&self, packet: &[u8], addr: &str) -> Result<()> {
        self.socket.send_to(packet, addr)?;

        Ok(())
    }
}
