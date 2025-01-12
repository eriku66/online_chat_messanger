mod chat_room;
mod chat_room_list;
mod chat_room_service;
mod user_session;
mod user_session_list;
use anyhow::{Context, Result};
use chat_room_service::ChatRoomService;
use shared::{
    operation_payload::OperationPayloadBuilder, ResponseStatus, TcpChatRoomPacket,
    TcpListenerWrapper, UdpMessagePacket, UserToken,
};
use std::{
    net::{TcpListener, UdpSocket},
    sync::Arc,
};
use tokio::sync::Mutex;

async fn handle_udp(chat_room_service: &ChatRoomService) -> Result<()> {
    let socket = UdpSocket::bind(shared::SERVER_ADDR_UDP)?;

    loop {
        let mut buf = [0; shared::MAX_MESSAGE_SIZE_BYTES];
        let (received, client_socket_addr) = socket
            .recv_from(&mut buf)
            .context("Failed to receive message")?;
        println!("Client socket address: {:?}", client_socket_addr);
        let udp_message_packet = UdpMessagePacket::from_packet(&buf[..received])?;
        println!("Packet: {:?}", udp_message_packet);

        chat_room_service
            .chat_room_list
            .get(&udp_message_packet.chat_room_name)?
            .user_session_list
            .send_to_all(
                &socket,
                udp_message_packet.message.value().as_bytes(),
                udp_message_packet.user_token,
                client_socket_addr,
            )?;
    }
}

async fn handle_tcp(chat_room_service: &mut ChatRoomService) -> Result<()> {
    let tcp_listener = TcpListenerWrapper::new(TcpListener::bind(shared::SERVER_ADDR_TCP)?);

    loop {
        let (mut tcp_stream, socket_addr) = tcp_listener.accept()?;

        let request_to_join_packet =
            TcpChatRoomPacket::from_bytes(&tcp_stream.read(TcpChatRoomPacket::MAX_BYTES)?)?;

        println!("Request to join packet: {:?}", request_to_join_packet);

        tcp_stream.write_all(
            &TcpChatRoomPacket::new(
                request_to_join_packet.room_name.clone(),
                request_to_join_packet.operation_type,
                shared::OperationState::ReceiveResponse,
                Some(
                    OperationPayloadBuilder::default()
                        .response_status(ResponseStatus::Ok)
                        .build()?,
                ),
            )
            .generate_bytes(),
        )?;

        let user_token = UserToken::default();

        if let Err(error) = chat_room_service.handle_request_to_join_packet(
            &request_to_join_packet,
            user_token.clone(),
            socket_addr,
        ) {
            tcp_stream.write_all(
                &TcpChatRoomPacket::new(
                    request_to_join_packet.room_name,
                    request_to_join_packet.operation_type,
                    shared::OperationState::CompleteResponse,
                    Some(
                        OperationPayloadBuilder::default()
                            .response_status(ResponseStatus::BadRequest)
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
                request_to_join_packet.room_name,
                request_to_join_packet.operation_type,
                shared::OperationState::CompleteResponse,
                Some(
                    OperationPayloadBuilder::default()
                        .response_status(ResponseStatus::Ok)
                        .user_token(user_token)
                        .build()?,
                ),
            )
            .generate_bytes(),
        )?;
    }
}

async fn start_server() -> Result<()> {
    let chat_room_service = Arc::new(Mutex::new(ChatRoomService::new()));

    let chat_room_service_clone = Arc::clone(&chat_room_service);

    let handle_tcp_task = tokio::spawn(async move {
        let mut tcp_chat_room_service = chat_room_service_clone.lock().await;
        handle_tcp(&mut tcp_chat_room_service)
            .await
            .unwrap_or_else(|err| {
                println!("Failed to handle TCP: {:?}", err);
            });
    });

    let chat_room_service_clone = Arc::clone(&chat_room_service);

    let handle_udp_task = tokio::spawn(async move {
        let udp_chat_room_service = chat_room_service_clone.lock().await;
        handle_udp(&udp_chat_room_service)
            .await
            .unwrap_or_else(|err| {
                println!("Failed to handle UDP: {:?}", err);
            });
    });

    tokio::try_join!(handle_tcp_task, handle_udp_task)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await?;

    Ok(())
}
