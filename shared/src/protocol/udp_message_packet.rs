use crate::{util::SliceUtil, ChatRoomName, Message, UserToken};
use anyhow::Result;

#[derive(Debug)]
pub struct UdpMessagePacket {
    pub chat_room_name: ChatRoomName,
    pub user_token: UserToken,
    pub message: Message,
}

impl UdpMessagePacket {
    pub const MAX_TOTAL_BYTES: usize =
        ChatRoomName::MAX_TOTAL_BYTES + UserToken::TOTAL_BYTES + Message::MAX_LENGTH;
    pub fn new(chat_room_name: ChatRoomName, user_token: UserToken, message: Message) -> Self {
        Self {
            chat_room_name,
            user_token,
            message,
        }
    }

    pub fn generate_packet(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        packet.push(self.chat_room_name.length as u8);
        packet.push(self.user_token.length as u8);
        packet.extend_from_slice(self.chat_room_name.value.as_bytes());
        packet.extend_from_slice(self.user_token.value.as_bytes());
        packet.extend_from_slice(self.message.value.as_bytes());

        packet
    }

    pub fn from_packet(packet: &[u8]) -> Result<Self> {
        let chat_room_name_length = u8::from_be_bytes([packet[0]]) as usize;
        let token_size_length = u8::from_be_bytes([packet[1]]) as usize;

        let splitted_packet =
            SliceUtil::split_slice(&packet[2..], &[chat_room_name_length, token_size_length]);

        let chat_room_name_str = String::from_utf8_lossy(&splitted_packet[0]).to_string();

        let user_token = String::from_utf8_lossy(&splitted_packet[1]).to_string();

        let message_str = String::from_utf8_lossy(&splitted_packet[2]).to_string();

        Ok(Self {
            chat_room_name: ChatRoomName::new(chat_room_name_str)?,
            user_token: UserToken::new(user_token),
            message: Message::new(message_str)?,
        })
    }
}
