use anyhow::{Context, Result};

use crate::{OperationState, OperationType, RoomName};

#[derive(Debug)]
pub struct TcpChatRoomPacket {
    pub room_name: RoomName,
    pub operation_type: OperationType,
    pub state: OperationState,
}

impl TcpChatRoomPacket {
    pub fn new(room_name: RoomName, operation_type: OperationType, state: OperationState) -> Self {
        Self {
            room_name,
            operation_type,
            state,
        }
    }

    pub fn generate_packet(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        packet.push(self.room_name.length() as u8);
        packet.push(self.operation_type as u8);
        packet.push(self.state as u8);
        packet.extend_from_slice(self.room_name.value().as_bytes());

        packet
    }

    pub fn from_packet(packet: &[u8]) -> Result<Self> {
        let _room_name_length = u8::from_be_bytes([packet[0]]) as usize;
        let operation_type = OperationType::from_u8(u8::from_be_bytes([packet[1]]))
            .context("Invalid operation type")?;
        let state =
            OperationState::from_u8(u8::from_be_bytes([packet[2]])).context("Invalid state")?;
        let room_name = String::from_utf8_lossy(&packet[3..]).to_string();

        Ok(Self {
            room_name: RoomName::new(room_name).unwrap(),
            operation_type,
            state,
        })
    }
}
