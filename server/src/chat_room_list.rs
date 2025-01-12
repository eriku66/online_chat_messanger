use anyhow::{anyhow, Context, Result};
use shared::{ChatRoomName, UserToken};
use std::collections::HashMap;

use crate::{chat_room::ChatRoom, user_session::UserSession};

#[derive(Debug, Default)]
pub struct ChatRoomList {
    list: HashMap<ChatRoomName, ChatRoom>,
}

impl ChatRoomList {
    fn exists(&self, chat_room_name: &ChatRoomName) -> bool {
        self.list.contains_key(chat_room_name)
    }

    pub fn create(
        &mut self,
        chat_room_name: ChatRoomName,
        host_user_token: UserToken,
        host_user_session: UserSession,
    ) -> Result<()> {
        if self.exists(&chat_room_name) {
            return Err(anyhow!("Chat room already exists"));
        }

        let chat_room = ChatRoom::new_with_host(host_user_token, host_user_session);

        self.list.insert(chat_room_name, chat_room);

        Ok(())
    }

    pub fn join(
        &mut self,
        chat_room_name: ChatRoomName,
        member_user_token: UserToken,
        user_session: UserSession,
    ) -> Result<()> {
        self.list
            .get_mut(&chat_room_name)
            .ok_or_else(|| anyhow!("Chat room does not exist"))?
            .add_member(member_user_token, user_session);

        Ok(())
    }

    pub fn get(&self, chat_room_name: &ChatRoomName) -> Result<&ChatRoom> {
        self.list.get(chat_room_name).context("Chat room not found")
    }
}
