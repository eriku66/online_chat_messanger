mod chat_room;
mod chat_room_list;
mod chat_room_service;
mod user_session;
mod user_session_list;

use anyhow::Result;
use chat_room_service::ChatRoomService;
use shared::{OperationPayload, ResponseStatus, TcpChatRoomPacket, TcpListenerWrapper, UserToken};
use std::net::TcpListener;

fn handle_tcp(chat_room_service: &mut ChatRoomService) -> Result<()> {
    let tcp_listener = TcpListenerWrapper::new(TcpListener::bind(shared::SERVER_ADDR)?);

    loop {
        let (mut tcp_stream, socket_addr) = tcp_listener.accept()?;

        let tcp_chat_room_packet =
            TcpChatRoomPacket::from_bytes(&tcp_stream.read(TcpChatRoomPacket::MAX_BYTES)?)?;

        println!("tcp_chat_room_packet: {:?}", tcp_chat_room_packet);

        tcp_stream.write_all(
            &TcpChatRoomPacket::new(
                tcp_chat_room_packet.room_name.clone(),
                tcp_chat_room_packet.operation_type,
                shared::OperationState::ReceiveResponse,
                Some(OperationPayload::new(ResponseStatus::Ok, None, None)),
            )
            .generate_bytes(),
        )?;

        let user_token = UserToken::default();

        if let Err(message) = chat_room_service.handle_chat_room_packet(
            &tcp_chat_room_packet,
            user_token.clone(),
            socket_addr,
        ) {
            println!("Error: {}", message);

            tcp_stream.write_all(
                &TcpChatRoomPacket::new(
                    tcp_chat_room_packet.room_name,
                    tcp_chat_room_packet.operation_type,
                    shared::OperationState::CompleteResponse,
                    Some(OperationPayload::new(
                        ResponseStatus::BadRequest,
                        Some(message.to_string()),
                        None,
                    )),
                )
                .generate_bytes(),
            )?;

            continue;
        };

        tcp_stream.write_all(
            &TcpChatRoomPacket::new(
                tcp_chat_room_packet.room_name,
                tcp_chat_room_packet.operation_type,
                shared::OperationState::CompleteResponse,
                Some(OperationPayload::new(
                    ResponseStatus::Ok,
                    None,
                    Some(user_token),
                )),
            )
            .generate_bytes(),
        )?;
    }
}

fn start_server() -> Result<()> {
    let mut chat_room_service = ChatRoomService::new();

    handle_tcp(&mut chat_room_service)?;

    Ok(())
}

fn main() -> Result<()> {
    start_server()?;

    Ok(())
}
