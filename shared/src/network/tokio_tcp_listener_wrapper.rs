use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use super::TokioTcpStreamWrapper;

pub struct TokioTcpListenerWrapper {
    tcp_listener: TcpListener,
}

impl TokioTcpListenerWrapper {
    pub fn new(tcp_listener: TcpListener) -> Self {
        Self { tcp_listener }
    }

    pub async fn accept(&self) -> Result<(TokioTcpStreamWrapper, SocketAddr)> {
        let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;
        Ok((TokioTcpStreamWrapper::new(tcp_stream), socket_addr))
    }
}
