use std::net::SocketAddr;
use std::time::Instant;

use shared::UserName;

#[derive(Debug)]
pub struct UserSession {
    pub socket_addr: SocketAddr,
    pub user_name: UserName,
    pub last_received_at: Instant,
}

impl UserSession {
    pub fn new(socket_addr: SocketAddr, user_name: UserName) -> Self {
        Self {
            socket_addr,
            user_name,
            last_received_at: Instant::now(),
        }
    }
}
