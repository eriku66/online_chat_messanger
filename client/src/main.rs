use anyhow::Result;
use shared::{ClientSocket, Message, UdpMessagePacket, UserName, UserSession};

fn prompt_user_name() -> String {
    println!("Please enter your name:");

    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();

    user_name
}

fn prompt_message() -> String {
    println!("Please enter your message:");

    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();

    message
}

fn start_session() -> Result<UserSession> {
    let user_name = UserName::new(prompt_user_name())?;
    let session = UserSession::new(ClientSocket::new()?, user_name);

    Ok(session)
}

fn main() -> Result<()> {
    let session = start_session()?;

    loop {
        let message = Message::new(prompt_message())?;

        let message_packet = UdpMessagePacket::new(session.user_name().clone(), message);

        println!("Sending message: {:?}", message_packet);

        session
            .client_socket()
            .send_to(&message_packet.generate_packet(), shared::SERVER_ADDR)
            .unwrap();
    }
}
