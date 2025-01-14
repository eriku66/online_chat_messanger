use anyhow::{Context, Result};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct TokioTcpStreamWrapper {
    tcp_stream: TcpStream,
}

impl TokioTcpStreamWrapper {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self { tcp_stream }
    }

    pub async fn read(&mut self, buf_size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; buf_size];
        let received = self.tcp_stream.read(&mut buf).await?;
        Ok(buf[..received].to_vec())
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.tcp_stream
            .write_all(buf)
            .await
            .context("Failed to write all")
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream
            .local_addr()
            .context("Failed to get local address")
    }
}
