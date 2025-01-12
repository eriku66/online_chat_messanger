use crate::client_socket::ClientSocket;
use shared::{ChatRoomName, UserToken};

#[derive(Debug)]
pub struct UserSession {
    pub client_socket: ClientSocket,
    pub chat_room_name: ChatRoomName,
    pub user_token: UserToken,
}

impl UserSession {
    pub fn new(
        client_socket: ClientSocket,
        chat_room_name: ChatRoomName,
        user_token: UserToken,
    ) -> Self {
        Self {
            client_socket,
            chat_room_name,
            user_token,
        }
    }
}
