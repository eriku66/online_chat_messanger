use crate::{ChatRoomName, Message, UserToken};
use anyhow::Result;

#[derive(Debug)]
pub struct UdpMessagePacket {
    pub chat_room_name: ChatRoomName,
    pub user_token: UserToken,
    pub message: Message,
}

impl UdpMessagePacket {
    pub fn new(chat_room_name: ChatRoomName, user_token: UserToken, message: Message) -> Self {
        Self {
            chat_room_name,
            user_token,
            message,
        }
    }

    pub fn generate_packet(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        packet.push(self.chat_room_name.length() as u8);
        packet.extend_from_slice(self.chat_room_name.value().as_bytes());
        packet.extend_from_slice(self.message.value().as_bytes());

        packet
    }

    pub fn from_packet(packet: &[u8]) -> Result<Self> {
        let chat_room_name_length = u8::from_be_bytes([packet[0]]) as usize;
        let token_size_length = u8::from_be_bytes([packet[1]]) as usize;

        let mut read_index = 2;

        let chat_room_name_str =
            String::from_utf8_lossy(&packet[read_index..(read_index + chat_room_name_length)])
                .to_string();

        read_index += chat_room_name_length;

        let user_token =
            String::from_utf8_lossy(&packet[read_index..(read_index + token_size_length)])
                .to_string();

        read_index += token_size_length;

        let message_str = String::from_utf8_lossy(&packet[read_index..]).to_string();

        Ok(Self {
            chat_room_name: ChatRoomName::new(chat_room_name_str)?,
            user_token: UserToken::new(user_token),
            message: Message::new(message_str)?,
        })
    }
}
