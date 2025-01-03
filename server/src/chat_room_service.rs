use std::net::SocketAddr;

use anyhow::{anyhow, Result};
use shared::{TcpChatRoomPacket, UserToken};

use crate::{chat_room_list::ChatRoomList, user_session::UserSession};

pub struct ChatRoomService {
    pub chat_room_list: ChatRoomList,
}

impl ChatRoomService {
    pub fn new() -> Self {
        Self {
            chat_room_list: ChatRoomList::default(),
        }
    }

    pub fn handle_request_to_join_packet(
        &mut self,
        packet: &TcpChatRoomPacket,
        user_token: UserToken,
        socket_addr: SocketAddr,
    ) -> Result<()> {
        let room_name = packet.room_name.clone();
        let user_name = packet
            .operation_payload
            .as_ref()
            .ok_or_else(|| anyhow!("Operation payload not found"))?
            .user_name
            .as_ref()
            .ok_or_else(|| anyhow!("User name not found"))?
            .clone();

        let user_session = UserSession::new(socket_addr, user_name);

        match packet.operation_type {
            shared::OperationType::CreateChatRoom => {
                self.chat_room_list
                    .create(room_name, user_token, user_session)?;
            }
            shared::OperationType::JoinChatRoom => {
                self.chat_room_list
                    .join(room_name, user_token, user_session)?;
            }
        }

        Ok(())
    }
}
