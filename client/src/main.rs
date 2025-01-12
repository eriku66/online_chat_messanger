mod client_socket;
mod prompts;
mod user_session;

use anyhow::{anyhow, Context, Result};
use client_socket::ClientSocket;
use shared::{
    operation_payload::OperationPayloadBuilder, ChatRoomName, Message, OperationState,
    OperationType, ResponseStatus, TcpChatRoomPacket, TcpStreamWrapper, UdpMessagePacket, UserName,
    UserToken,
};
use std::{net::TcpStream, sync::Arc};
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

    let message_packet = UdpMessagePacket::new(
        session.chat_room_name.clone(),
        session.user_token.clone(),
        message,
    );

    session
        .client_socket
        .send_to(&message_packet.generate_packet(), shared::SERVER_ADDR_UDP)
        .await?;

    Ok(())
}

fn extract_user_token(response_for_task_complete: TcpChatRoomPacket) -> UserToken {
    response_for_task_complete
        .operation_payload
        .unwrap()
        .user_token
        .unwrap()
}

fn validate_task_completion_response(response_for_task_complete: &TcpChatRoomPacket) -> Result<()> {
    let response_for_task_complete_payload = response_for_task_complete
        .operation_payload
        .as_ref()
        .ok_or_else(|| anyhow!("Response for task complete payload not found"))?;

    if response_for_task_complete.state != OperationState::CompleteResponse {
        return Err(anyhow::anyhow!("Operation state is not matching"));
    }

    if response_for_task_complete_payload.response_status != Some(ResponseStatus::Ok) {
        if let Some(message) = &response_for_task_complete_payload.message {
            return Err(anyhow::anyhow!("Error message: {}", message));
        }

        return Err(anyhow::anyhow!("Response status is not OK"));
    }

    if response_for_task_complete_payload.user_token.is_none() {
        return Err(anyhow::anyhow!("User token not found"));
    }

    Ok(())
}

fn validate_receive_request_response(
    response_for_receiving_request: &TcpChatRoomPacket,
) -> Result<()> {
    if !(response_for_receiving_request.state == OperationState::ReceiveResponse
        && response_for_receiving_request
            .operation_payload
            .as_ref()
            .ok_or_else(|| anyhow!("Response for receiving payload not found"))?
            .response_status
            == Some(ResponseStatus::Ok))
    {
        return Err(anyhow::anyhow!("Server rejected request"));
    }

    Ok(())
}

fn request_to_join_chat_room(
    tcp_stream: &mut TcpStreamWrapper,
    room_name: &ChatRoomName,
) -> Result<()> {
    let operation_type = OperationType::try_from_u8(
        prompt(prompts::CREATE_OR_JOIN_PROMPT)
            .trim()
            .parse::<u8>()
            .context("Input must be a number")?,
    )
    .context("Invalid operation type")?;
    let user_name = UserName::new(prompt(prompts::USER_NAME_PROMPT))?;

    let chat_room_packet = TcpChatRoomPacket::new(
        room_name.clone(),
        operation_type,
        OperationState::Request,
        Some(
            OperationPayloadBuilder::default()
                .user_name(user_name)
                .build()?,
        ),
    );

    tcp_stream.write_all(&chat_room_packet.generate_bytes())?;

    Ok(())
}

fn join_chat_room(
    tcp_stream: &mut TcpStreamWrapper,
    room_name: &ChatRoomName,
) -> Result<UserToken> {
    request_to_join_chat_room(tcp_stream, room_name)?;

    let receiving_request_response =
        TcpChatRoomPacket::from_bytes(&tcp_stream.read(TcpChatRoomPacket::MAX_BYTES)?)?;

    validate_receive_request_response(&receiving_request_response)?;

    let task_completion_response =
        TcpChatRoomPacket::from_bytes(&tcp_stream.read(TcpChatRoomPacket::MAX_BYTES)?)?;

    validate_task_completion_response(&task_completion_response)?;

    Ok(extract_user_token(task_completion_response))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut tcp_stream = TcpStreamWrapper::new(TcpStream::connect(shared::SERVER_ADDR_TCP)?);
    let room_name = ChatRoomName::new(prompt(prompts::ROOM_NAME_PROMPT))?;

    let user_token = join_chat_room(&mut tcp_stream, &room_name).map_err(|join_chat_room_err| {
        println!("Failed to join chat room: {:?}", join_chat_room_err);

        let _ = tcp_stream.shutdown();

        join_chat_room_err
    })?;

    println!("User token: {:?}", user_token);

    let session = Arc::new(UserSession::new(
        ClientSocket::new(tcp_stream.local_addr()?)?,
        room_name,
        user_token,
    ));

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
