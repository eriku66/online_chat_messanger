mod client_socket;
mod prompts;
mod user_session;

use anyhow::Result;
use client_socket::ClientSocket;
use shared::{Message, UdpMessagePacket, UserName};
use std::sync::Arc;
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

#[tokio::main]
async fn main() -> Result<()> {
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
