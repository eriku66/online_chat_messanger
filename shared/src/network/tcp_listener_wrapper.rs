use std::net::{SocketAddr, TcpListener};

use super::TcpStreamWrapper;

pub struct TcpListenerWrapper {
    tcp_listener: TcpListener,
}

impl TcpListenerWrapper {
    pub fn new(tcp_listener: TcpListener) -> Self {
        Self { tcp_listener }
    }

    pub fn accept(&self) -> std::io::Result<(TcpStreamWrapper, SocketAddr)> {
        let (tcp_stream, socket_addr) = self.tcp_listener.accept()?;
        Ok((TcpStreamWrapper::new(tcp_stream), socket_addr))
    }
}
