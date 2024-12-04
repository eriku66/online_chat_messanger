use anyhow::Result;
use std::net::UdpSocket;

pub struct ClientSocket {
    pub socket: std::net::UdpSocket,
}

impl ClientSocket {
    pub fn new() -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind("127.0.0.1:0").unwrap(),
        })
    }
}
