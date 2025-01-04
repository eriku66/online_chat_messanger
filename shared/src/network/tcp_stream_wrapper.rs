use anyhow::{Context, Result};
use std::{
    io::{Read, Write},
    net::{Shutdown, SocketAddr, TcpStream},
};

pub struct TcpStreamWrapper {
    tcp_stream: TcpStream,
}

impl TcpStreamWrapper {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self { tcp_stream }
    }

    pub fn read(&mut self, buf_size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; buf_size];
        let received = self.tcp_stream.read(&mut buf)?;
        Ok(buf[..received].to_vec())
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.tcp_stream
            .write_all(buf)
            .context("Failed to write all")
    }

    pub fn shutdown(&mut self) -> Result<()> {
        if self.tcp_stream.peer_addr().is_err() {
            return Ok(());
        }

        self.tcp_stream
            .shutdown(Shutdown::Both)
            .context("Failed to shutdown")
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream
            .local_addr()
            .context("Failed to get local address")
    }
}
