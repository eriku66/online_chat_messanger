use crate::{OperationPayload, OperationState, OperationType, RoomName};
use anyhow::{Context, Result};
use std::{io::Read, net::TcpStream};

#[derive(Debug)]
pub struct TcpChatRoomPacket {
    pub room_name: RoomName,
    pub operation_type: OperationType,
    pub state: OperationState,
}

impl TcpChatRoomPacket {
    const MAX_BYTES: usize = RoomName::HEADER_LENGTH_BYTES
        + OperationType::HEADER_LENGTH_BYTES
        + OperationState::HEADER_LENGTH_BYTES
        + OperationPayload::HEADER_LENGTH_BYTES;

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

    pub fn from_tcp_stream(tcp_stream: &mut TcpStream) -> Result<Self> {
        let mut buf = [0; Self::MAX_BYTES];
        let received = tcp_stream
            .read(&mut buf)
            .context("Failed to read from TCP stream")?;

        let packet = &buf[..received];

        let room_name_length = u8::from_be_bytes([packet[0]]) as usize;
        let operation_type = OperationType::from_u8(u8::from_be_bytes([packet[1]]))
            .context("Invalid operation type")?;
        let state =
            OperationState::from_u8(u8::from_be_bytes([packet[2]])).context("Invalid state")?;
        let body = String::from_utf8_lossy(&packet[3..]).to_string();

        let (room_name, _) = body.split_at(room_name_length);

        Ok(Self {
            room_name: RoomName::new(room_name.to_string()).unwrap(),
            operation_type,
            state,
        })
    }
}
