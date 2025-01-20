use std::net::{SocketAddr, UdpSocket};

use anyhow::Result;
use tokio::net::UdpSocket as TokioUdpSocket;

#[derive(Debug)]
pub struct ClientSocket {
    pub socket: TokioUdpSocket,
}

impl ClientSocket {
    pub fn new(socket_addr: SocketAddr) -> Result<Self> {
        Ok(Self {
            socket: TokioUdpSocket::from_std(UdpSocket::bind(socket_addr)?)?,
        })
    }

    pub async fn send_to(&self, packet: &[u8], addr: &str) -> Result<()> {
        self.socket.send_to(packet, addr).await?;

        Ok(())
    }
}
