use anyhow::Result;

use crate::ClientSocket;
use crate::UdpSessionStartPacket;
use crate::UserName;
use crate::SERVER_ADDR;

#[derive(Debug)]
pub struct UserSession {
    pub client_socket: ClientSocket,
    pub user_name: UserName,
}

impl UserSession {
    pub fn new(client_socket: ClientSocket, user_name: UserName) -> Self {
        Self {
            client_socket,
            user_name,
        }
    }

    pub fn start(&self) -> Result<()> {
        let packet = UdpSessionStartPacket::new(self.user_name.clone()).generate_packet();
        self.client_socket.send_to(&packet, SERVER_ADDR).unwrap();

        Ok(())
    }

    pub fn client_socket(&self) -> &ClientSocket {
        &self.client_socket
    }

    pub fn user_name(&self) -> &UserName {
        &self.user_name
    }
}
