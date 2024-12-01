use crate::{Message, UserName};
use anyhow::Result;

#[derive(Debug)]
pub struct UdpMessagePacket {
    pub user_name: UserName,
    pub message: Message,
}

impl UdpMessagePacket {
    pub fn new(user_name: UserName, message: Message) -> Self {
        Self { user_name, message }
    }

    pub fn generate_packet(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        packet.push(self.user_name.length() as u8);
        packet.extend_from_slice(self.user_name.value().as_bytes());
        packet.extend_from_slice(self.message.value().as_bytes());

        packet
    }

    pub fn from_packet(packet: &[u8]) -> Result<Self> {
        let user_name_length = u8::from_be_bytes([packet[0]]) as usize;
        let user_name_and_message = String::from_utf8_lossy(&packet[1..]).to_string();
        let (user_name, message) = user_name_and_message.split_at(user_name_length);

        Ok(Self {
            user_name: UserName::new(user_name.to_string())?,
            message: Message::new(message.to_string())?,
        })
    }
}
