mod client_socket;
mod consts;
mod prompts;
mod user_session;

use anyhow::{Context, Result};
use client_socket::ClientSocket;
use consts::CLIENT_ADDR;
use shared::{
    ChatRoomName, Message, OperationState, OperationType, TcpChatRoomPacket, UdpMessagePacket,
    UserName,
};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    process::exit,
    sync::Arc,
};
use tokio::net::tcp;
use user_session::UserSession;

fn prompt(message_prompt: &str) -> String {
    println!("{}", message_prompt);

    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();

    message
}

async fn receive_message(session: &UserSession) -> Result<String> {
    let mut buf = [0; shared::MAX_MESSAGE_SIZE_BYTES];

    let (received, _) = session.client_socket.socket.recv_from(&mut buf).await?;

    Ok(String::from_utf8_lossy(&buf[..received]).to_string())
}

async fn send_message(session: &UserSession) -> Result<()> {
    let message = Message::new(prompt(prompts::MESSAGE_PROMPT))?;

    let message_packet = UdpMessagePacket::new(session.user_name.clone(), message);

    session
        .client_socket
        .send_to(&message_packet.generate_packet(), shared::SERVER_ADDR)
        .await?;

    Ok(())
}

fn start_session() -> Result<UserSession> {
    let user_name = UserName::new(prompt(prompts::USER_NAME_PROMPT))?;
    let session = UserSession::new(ClientSocket::new()?, user_name);

    Ok(session)
}

fn join_chat_room() -> Result<()> {
    let room_name = ChatRoomName::new(prompt(prompts::ROOM_NAME_PROMPT))?;
    let operation_type = OperationType::from_u8(
        prompt(prompts::CREATE_OR_JOIN_PROMPT)
            .trim()
            .parse::<u8>()
            .context("Input must be a number")?,
    )
    .context("Invalid operation type")?;

    let chat_room_packet =
        TcpChatRoomPacket::new(room_name, operation_type, OperationState::Request, None);

    let mut tcp_stream = TcpStream::connect(shared::SERVER_ADDR)?;
    tcp_stream.write_all(&chat_room_packet.generate_packet())?;

    let tcp_listener = TcpListener::bind(tcp_stream.local_addr()?)?;

    let (mut server_tcp_stream, _) = tcp_listener.accept()?;

    let response_packet = TcpChatRoomPacket::from_tcp_stream(&mut server_tcp_stream)?;

    println!("Response packet: {:?}", response_packet);

    exit(0);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    join_chat_room()?;
    let session = Arc::new(start_session()?);

    let send_session = session.clone();
    let receive_session = session.clone();

    let send_task = tokio::spawn(async move {
        loop {
            send_message(&send_session).await.unwrap_or_else(|err| {
                println!("Failed to send message: {:?}", err);
            });
        }
    });

    let receive_task = tokio::spawn(async move {
        loop {
            if let Ok(message) = receive_message(&receive_session).await {
                println!(
                    "Received message: {:?} \n{}",
                    message,
                    prompts::MESSAGE_PROMPT
                );
            } else {
                println!("Failed to receive message");
            }
        }
    });

    tokio::try_join!(receive_task, send_task)?;

    Ok(())
}
