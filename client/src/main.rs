use anyhow::Result;
use shared::{Message, UdpMessagePacket, UserName};
use std::net::UdpSocket;

fn prompt_user_name() -> String {
    println!("Please enter your name:");

    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();

    user_name
}

fn prompt_message() -> String {
    println!("Please enter your message:");

    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();

    user_name
}

fn main() -> Result<()> {
    let message_packet = UdpMessagePacket::new(
        UserName::new(prompt_user_name())?,
        Message::new(prompt_message())?,
    );

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket
        .send_to(&message_packet.generate_packet(), shared::SERVER_ADDR)
        .unwrap();

    Ok(())
}
