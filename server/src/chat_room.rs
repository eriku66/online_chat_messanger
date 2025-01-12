use crate::{user_session::UserSession, user_session_list::UserSessionList, UserToken};

#[derive(Debug)]
pub struct ChatRoom {
    _host_user_token: UserToken,
    pub user_session_list: UserSessionList,
}

impl ChatRoom {
    pub fn new_with_host(host_user_token: UserToken, host_user_session: UserSession) -> Self {
        let mut user_session_list = UserSessionList::default();
        user_session_list.add_or_update(host_user_token.clone(), host_user_session);

        Self {
            _host_user_token: host_user_token,
            user_session_list,
        }
    }

    pub fn add_member(&mut self, user_token: UserToken, user_session: UserSession) {
        self.user_session_list
            .add_or_update(user_token, user_session);
    }
}
