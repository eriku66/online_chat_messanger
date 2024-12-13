use super::user_session::UserSession;
use std::net::SocketAddr;

#[derive(Debug, Default)]
pub struct UserSessionList {
    pub list: Vec<UserSession>,
}

impl UserSessionList {
    pub fn add(&mut self, client_socket_address: SocketAddr) {
        self.list.push(UserSession::new(client_socket_address));
    }
}
