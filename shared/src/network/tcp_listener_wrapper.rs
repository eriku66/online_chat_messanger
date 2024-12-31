use std::net::TcpListener;

use super::TcpStreamWrapper;

pub struct TcpListenerWrapper {
    tcp_listener: TcpListener,
}

impl TcpListenerWrapper {
    pub fn new(tcp_listener: TcpListener) -> Self {
        Self { tcp_listener }
    }

    pub fn accept(&self) -> std::io::Result<TcpStreamWrapper> {
        let (tcp_stream, _socket_addr) = self.tcp_listener.accept()?;
        Ok(TcpStreamWrapper::new(tcp_stream))
    }
}
