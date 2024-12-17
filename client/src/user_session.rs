use crate::{client_socket::ClientSocket, UserName};

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
}
