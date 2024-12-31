mod chat_room;
mod chat_room_list;
mod chat_room_service;
mod user_session;
mod user_session_list;

use anyhow::{Context, Result};
use chat_room_service::ChatRoomService;
use shared::{OperationPayload, ResponseStatus, TcpChatRoomPacket, UdpMessagePacket};
use std::{
    io::Write,
    net::{TcpListener, UdpSocket},
    thread::sleep,
};
use user_session_list::UserSessionList;

fn create_socket() -> std::io::Result<UdpSocket> {
    UdpSocket::bind(shared::SERVER_ADDR)
}

fn handle_socket(socket: UdpSocket) -> Result<()> {
    let mut user_session_list = UserSessionList::default();

    loop {
        let mut buf = [0; shared::MAX_MESSAGE_SIZE_BYTES];
        let (received, client_socket_addr) = socket
            .recv_from(&mut buf)
            .context("Failed to receive message")?;

        let udp_message_packet = UdpMessagePacket::from_packet(&buf[..received])?;

        user_session_list.add_or_update(client_socket_addr);

        user_session_list.cleanup();

        println!("User session list: {:?}", user_session_list);

        user_session_list.send_to_all(
            &socket,
            udp_message_packet.message.value().as_bytes(),
            client_socket_addr,
        )?;
    }
}

fn start_server() -> Result<()> {
    let tcp_listener = TcpListener::bind(shared::SERVER_ADDR)?;

    let mut chat_room_service = ChatRoomService::new();

    loop {
        let (mut tcp_stream, _client_tcp_socket_addr) = tcp_listener.accept()?;

        let tcp_chat_room_packet = TcpChatRoomPacket::from_tcp_stream(&mut tcp_stream)?;

        println!("tcp_chat_room_packet: {:?}", tcp_chat_room_packet);
        println!("client_tcp_socket_addr: {:?}", _client_tcp_socket_addr);

        tcp_stream.write_all(
            &TcpChatRoomPacket::new(
                tcp_chat_room_packet.room_name.clone(),
                tcp_chat_room_packet.operation_type,
                shared::OperationState::ReceiveResponse,
                Some(OperationPayload::new(ResponseStatus::Ok, None)),
            )
            .generate_bytes(),
        )?;

        sleep(std::time::Duration::from_secs(1));

        if let Err(message) = chat_room_service.handle_chat_room_packet(&tcp_chat_room_packet) {
            println!("Error: {}", message);

            tcp_stream.write_all(
                &TcpChatRoomPacket::new(
                    tcp_chat_room_packet.room_name,
                    tcp_chat_room_packet.operation_type,
                    shared::OperationState::CompleteResponse,
                    Some(OperationPayload::new(
                        ResponseStatus::BadRequest,
                        Some(message.to_string()),
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
                Some(OperationPayload::new(ResponseStatus::Ok, None)),
            )
            .generate_bytes(),
        )?;
    }

    let socket = create_socket()?;
    handle_socket(socket)?;

    Ok(())
}

fn main() -> Result<()> {
    start_server()?;

    Ok(())
}
