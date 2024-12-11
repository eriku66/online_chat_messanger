use std::net::SocketAddr;

#[derive(Default)]
pub struct UserSessionList {
    pub list: Vec<SocketAddr>,
}

impl UserSessionList {
    pub fn add(&mut self, client_socket_address: SocketAddr) {
        self.list.push(client_socket_address);
    }
}
