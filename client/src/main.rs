use std::net::UdpSocket;

fn main() {
    println!("Please enter your name:");

    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();

    if user_name.as_bytes().len() > shared::MAX_USER_NAME_SIZE_BYTES {
        eprintln!(
            "User name must be less than or equal to {} bytes",
            shared::MAX_USER_NAME_SIZE_BYTES
        );

        return;
    }

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket
        .send_to(user_name.as_bytes(), shared::SERVER_ADDR)
        .unwrap();
}
