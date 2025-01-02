use std::net::SocketAddr;

use anyhow::Result;
use shared::{TcpChatRoomPacket, UserToken};

use crate::chat_room_list::ChatRoomList;

pub struct ChatRoomService {
    pub chat_room_list: ChatRoomList,
}

impl ChatRoomService {
    pub fn new() -> Self {
        Self {
            chat_room_list: ChatRoomList::default(),
        }
    }

    pub fn handle_chat_room_packet(
        &mut self,
        packet: &TcpChatRoomPacket,
        user_token: UserToken,
        socket_addr: SocketAddr,
    ) -> Result<()> {
        match packet.operation_type {
            shared::OperationType::CreateChatRoom => {
                self.chat_room_list
                    .create(packet.room_name.clone(), user_token, socket_addr)?;
            }
            shared::OperationType::JoinChatRoom => {
                self.chat_room_list
                    .join(packet.room_name.clone(), user_token, socket_addr)?;
            }
        }

        Ok(())
    }
}
