use std::net::SocketAddr;
use std::time::Instant;

#[derive(Debug)]
pub struct UserSession {
    pub socket_addr: SocketAddr,
    pub last_received_at: Instant,
}

impl UserSession {
    pub fn new(socket_addr: SocketAddr) -> Self {
        Self {
            socket_addr,
            last_received_at: Instant::now(),
        }
    }
}
