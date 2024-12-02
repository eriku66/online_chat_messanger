use crate::UserName;
use anyhow::Result;

#[derive(Debug)]
pub struct UdpSessionStartPacket {
    pub user_name: UserName,
}

impl UdpSessionStartPacket {
    pub fn new(user_name: UserName) -> Self {
        Self { user_name }
    }

    pub fn generate_packet(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        packet.extend_from_slice(self.user_name.value().as_bytes());

        packet
    }

    pub fn from_packet(packet: &[u8]) -> Result<Self> {
        let user_name = String::from_utf8_lossy(packet).to_string();

        Ok(Self {
            user_name: UserName::new(user_name)?,
        })
    }
}
