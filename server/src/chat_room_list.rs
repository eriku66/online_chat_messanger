use std::collections::HashMap;

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

    pub fn create(&mut self, chat_room_name: ChatRoomName) {
        self.list.insert(chat_room_name, ChatRoom {});
    }

    pub fn join(&mut self, chat_room_name: ChatRoomName) {
        if !self.exists(&chat_room_name) {
            self.list.insert(chat_room_name.clone(), ChatRoom {});
        }
    }
}
