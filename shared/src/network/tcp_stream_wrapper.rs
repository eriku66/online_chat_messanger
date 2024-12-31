use std::{
    io::{Read, Write},
    net::TcpStream,
};

use anyhow::Result;

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

    pub fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.tcp_stream.write_all(buf)
    }
}
