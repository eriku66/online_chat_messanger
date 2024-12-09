use crate::ClientSocket;
use crate::UserName;

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

    pub fn client_socket(&self) -> &ClientSocket {
        &self.client_socket
    }

    pub fn user_name(&self) -> &UserName {
        &self.user_name
    }
}
