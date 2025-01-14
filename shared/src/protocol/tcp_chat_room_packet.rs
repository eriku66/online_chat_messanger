use crate::{ChatRoomName, OperationPayload, OperationState, OperationType};
use anyhow::Result;

#[derive(Debug)]
pub struct TcpChatRoomPacket {
    pub room_name: ChatRoomName,
    pub operation_type: OperationType,
    pub state: OperationState,
    pub operation_payload: Option<OperationPayload>,
}

impl TcpChatRoomPacket {
    pub const MAX_BYTES: usize = ChatRoomName::HEADER_LENGTH_BYTES
        + OperationType::HEADER_LENGTH_BYTES
        + OperationState::HEADER_LENGTH_BYTES
        + OperationPayload::MAX_TOTAL_BYTES;

    pub fn new(
        room_name: ChatRoomName,
        operation_type: OperationType,
        state: OperationState,
        operation_payload: Option<OperationPayload>,
    ) -> Self {
        Self {
            room_name,
            operation_type,
            state,
            operation_payload,
        }
    }

    pub fn generate_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        packet.push(self.room_name.length as u8);
        packet.push(self.operation_type as u8);
        packet.push(self.state as u8);
        packet.extend_from_slice(self.room_name.value.as_bytes());
        packet.extend_from_slice(
            serde_json::to_string(&self.operation_payload)
                .unwrap()
                .as_bytes(),
        );

        packet
    }

    pub fn from_bytes(packet: &[u8]) -> Result<Self> {
        let room_name_length = u8::from_be_bytes([packet[0]]) as usize;
        let operation_type = OperationType::try_from_u8(u8::from_be_bytes([packet[1]]))?;
        let state = OperationState::try_from_u8(u8::from_be_bytes([packet[2]]))?;
        let body = String::from_utf8_lossy(&packet[3..]).to_string();

        let (room_name, operation_payload) = body.split_at(room_name_length);

        Ok(Self {
            room_name: ChatRoomName::new(room_name.to_string())?,
            operation_type,
            state,
            operation_payload: serde_json::from_str(operation_payload).ok(),
        })
    }
}
