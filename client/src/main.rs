use anyhow::Result;
use shared::{ClientSocket, Message, UdpMessagePacket, UdpSessionStartPacket, UserName};
use std::net::UdpSocket;

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

fn start_session(user_name: UserName) -> Result<UdpSocket> {
    let socket = ClientSocket::new()?.socket;

    socket.send_to(
        &UdpSessionStartPacket::new(user_name).generate_packet(),
        shared::SERVER_ADDR,
    )?;

    Ok(socket)
}

fn main() -> Result<()> {
    let user_name = UserName::new(prompt_user_name())?;

    let socket = start_session(user_name.clone())?;

    loop {
        let message = Message::new(prompt_message())?;

        let message_packet = UdpMessagePacket::new(user_name.clone(), message);

        println!("Sending message: {:?}", message_packet);

        socket
            .send_to(&message_packet.generate_packet(), shared::SERVER_ADDR)
            .unwrap();
    }
}
