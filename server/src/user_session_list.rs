use anyhow::Result;
use shared::SESSION_TIMEOUT_SECONDS;

use super::user_session::UserSession;
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

#[derive(Debug, Default)]
pub struct UserSessionList {
    list: HashMap<SocketAddr, UserSession>,
}

impl UserSessionList {
    pub fn add_or_update(&mut self, client_socket_address: SocketAddr) {
        if let Some(user_session) = self.list.get_mut(&client_socket_address) {
            user_session.last_received_at = std::time::Instant::now();

            return;
        }

        self.list.insert(client_socket_address, UserSession::new());
    }

    pub fn send_to_all(
        &self,
        socket: &UdpSocket,
        packet: &[u8],
        excepted_addr: SocketAddr,
    ) -> Result<()> {
        for (&socket_addr, _) in self.list.iter() {
            if socket_addr == excepted_addr {
                continue;
            }

            socket.send_to(packet, socket_addr)?;
        }

        Ok(())
    }

    pub fn cleanup(&mut self) {
        self.list.retain(|_, user_session| {
            user_session.last_received_at.elapsed().as_secs() < SESSION_TIMEOUT_SECONDS
        });
    }
}
