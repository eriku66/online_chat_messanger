use std::collections::HashMap;

use anyhow::{anyhow, Result};
use shared::ChatRoomName;

use crate::chat_room::ChatRoom;

#[derive(Debug, Default)]
pub struct ChatRoomList {
    list: HashMap<ChatRoomName, ChatRoom>,
}

impl ChatRoomList {
    fn exists(&self, chat_room_name: &ChatRoomName) -> bool {
        self.list.contains_key(chat_room_name)
    }

    pub fn create(&mut self, chat_room_name: ChatRoomName) -> Result<()> {
        if self.exists(&chat_room_name) {
            return Err(anyhow!("Chat room already exists"));
        }

        self.list.insert(chat_room_name, ChatRoom {});

        Ok(())
    }

    pub fn join(&mut self, chat_room_name: ChatRoomName) -> Result<()> {
        if !self.exists(&chat_room_name) {
            return Err(anyhow!("Chat room does not exist"));
        }

        self.list.insert(chat_room_name.clone(), ChatRoom {});

        Ok(())
    }
}
