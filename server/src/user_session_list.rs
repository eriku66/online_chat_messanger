use super::user_session::UserSession;
use anyhow::{anyhow, Result};
use shared::{UserToken, SESSION_TIMEOUT_SECONDS};
use std::{collections::HashMap, net::SocketAddr};
use tokio::net::UdpSocket;

#[derive(Debug, Default)]
pub struct UserSessionList {
    list: HashMap<UserToken, UserSession>,
}

impl UserSessionList {
    pub fn add_or_update(&mut self, user_token: UserToken, user_session: UserSession) {
        if let Some(user_session) = self.list.get_mut(&user_token) {
            user_session.last_received_at = std::time::Instant::now();

            return;
        }

        self.list.insert(user_token, user_session);
    }

    pub async fn send_to_all(
        &self,
        socket: &UdpSocket,
        packet: &[u8],
        sender_user_token: UserToken,
        sender_socket_addr: SocketAddr,
    ) -> Result<()> {
        if self
            .list
            .get(&sender_user_token)
            .ok_or_else(|| anyhow!("User session not found"))?
            .socket_addr
            != sender_socket_addr
        {
            return Err(anyhow!("User token and socket address mismatch"));
        }

        for (_, user_session) in self.list.iter() {
            println!("Sending to {:?}", user_session);
            if user_session.socket_addr == sender_socket_addr {
                continue;
            }

            socket.send_to(packet, user_session.socket_addr).await?;
        }

        Ok(())
    }

    pub fn _cleanup(&mut self) {
        self.list.retain(|_, user_session| {
            user_session.last_received_at.elapsed().as_secs() < SESSION_TIMEOUT_SECONDS
        });
    }
}
