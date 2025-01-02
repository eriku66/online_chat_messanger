use std::{collections::HashMap, net::SocketAddr};

use anyhow::{anyhow, Result};
use shared::{ChatRoomName, UserToken};

use crate::chat_room::ChatRoom;

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
        socket_addr: SocketAddr,
    ) -> Result<()> {
        if self.exists(&chat_room_name) {
            return Err(anyhow!("Chat room already exists"));
        }

        let chat_room = ChatRoom::new(host_user_token, socket_addr);

        self.list.insert(chat_room_name, chat_room);

        Ok(())
    }

    pub fn join(
        &mut self,
        chat_room_name: ChatRoomName,
        member_user_token: UserToken,
        socket_addr: SocketAddr,
    ) -> Result<()> {
        self.list
            .get_mut(&chat_room_name)
            .ok_or_else(|| anyhow!("Chat room does not exist"))?
            .add_member(member_user_token, socket_addr);

        Ok(())
    }
}
