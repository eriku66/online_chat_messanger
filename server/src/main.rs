mod chat_room;
mod chat_room_list;
mod chat_room_service;
mod user_session;
mod user_session_list;
use anyhow::Result;
use chat_room_service::ChatRoomService;
use shared::{
    operation_payload::OperationPayloadBuilder, ResponseStatus, TcpChatRoomPacket,
    TcpListenerWrapper, UserToken,
};
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
                Some(
                    OperationPayloadBuilder::default()
                        .status(ResponseStatus::Ok)
                        .build()?,
                ),
            )
            .generate_bytes(),
        )?;

        let user_token = UserToken::default();

        if let Err(error) = chat_room_service.handle_chat_room_packet(
            &tcp_chat_room_packet,
            user_token.clone(),
            socket_addr,
        ) {
            tcp_stream.write_all(
                &TcpChatRoomPacket::new(
                    tcp_chat_room_packet.room_name,
                    tcp_chat_room_packet.operation_type,
                    shared::OperationState::CompleteResponse,
                    Some(
                        OperationPayloadBuilder::default()
                            .status(ResponseStatus::BadRequest)
                            .message(error.to_string())
                            .build()?,
                    ),
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
                Some(
                    OperationPayloadBuilder::default()
                        .status(ResponseStatus::Ok)
                        .user_token(user_token)
                        .build()?,
                ),
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
