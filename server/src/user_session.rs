use std::net::SocketAddr;

#[derive(Debug)]
pub struct UserSession {
    pub client_socket_addr: SocketAddr,
    last_received_at: std::time::Instant,
}

impl UserSession {
    pub fn new(client_socket_addr: SocketAddr) -> Self {
        Self {
            client_socket_addr,
            last_received_at: std::time::Instant::now(),
        }
    }
}
