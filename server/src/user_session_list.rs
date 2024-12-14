use anyhow::Result;

use super::user_session::UserSession;
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug, Default)]
pub struct UserSessionList {
    pub list: Vec<UserSession>,
}

impl UserSessionList {
    pub fn add(&mut self, client_socket_address: SocketAddr) {
        self.list.push(UserSession::new(client_socket_address));
    }

    pub fn send_to_all(
        &self,
        socket: &UdpSocket,
        packet: &[u8],
        excepted_addr: SocketAddr,
    ) -> Result<()> {
        for user_session in self.list.iter() {
            if user_session.client_socket_addr == excepted_addr {
                continue;
            }

            socket.send_to(packet, user_session.client_socket_addr)?;
        }

        Ok(())
    }
}
