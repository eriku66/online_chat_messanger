use crate::UserName;
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug)]
pub struct UserSession {
    pub client_socket: UdpSocket,
    pub user_name: UserName,
}

impl UserSession {
    pub fn new(client_socket: UdpSocket, user_name: UserName) -> Self {
        Self {
            client_socket,
            user_name,
        }
    }

    pub fn client_socket(&self) -> &UdpSocket {
        &self.client_socket
    }

    pub fn user_name(&self) -> &UserName {
        &self.user_name
    }
}
