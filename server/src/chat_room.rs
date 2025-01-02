use crate::{user_session_list::UserSessionList, UserToken};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct ChatRoom {
    host_user_token: UserToken,
    user_session_list: UserSessionList,
}

impl ChatRoom {
    pub fn new(host_user_token: UserToken, socket_addr: SocketAddr) -> Self {
        let mut user_session_list = UserSessionList::default();
        user_session_list.add_or_update(host_user_token.clone(), socket_addr);

        Self {
            host_user_token,
            user_session_list,
        }
    }

    pub fn add_member(&mut self, user_token: UserToken, socket_addr: SocketAddr) {
        self.user_session_list
            .add_or_update(user_token, socket_addr);
    }
}
