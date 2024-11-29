use anyhow::Result;
use shared::UserName;
use std::net::UdpSocket;

fn prompt_user_name() -> String {
    println!("Please enter your name:");

    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();

    user_name
}

fn get_user_name() -> Result<UserName> {
    UserName::new(prompt_user_name())
}

fn main() {
    let user_name = get_user_name().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket
        .send_to(user_name.value().as_bytes(), shared::SERVER_ADDR)
        .unwrap();
}
