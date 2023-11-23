use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

use log::info;

mod argument;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let arguments = argument::parse_arguments();
    info!(
        "Packet loss rate is configured to be {}",
        arguments.loss_rate
    );
    info!("Listening on {}", arguments.bind_address);
    info!("Forwarding to {}", arguments.forward_address);
    // Start the listener
    let mut buffer = [0; 32 * 1024];
    let mut sender = SocketAddr::from_str("127.0.0.1:0").unwrap();
    let socket = UdpSocket::bind(arguments.bind_address).expect("Cannot listen on UDP socket");
    loop {
        let (n, src) = socket.recv_from(&mut buffer).expect("Cannot receive data");
        // Maybe drop this packet?
        if should_drop(arguments.loss_rate) {
            continue;
        }
        // Two possibilities...
        if src == arguments.forward_address { // either this packet is coming from server
            socket
                .send_to(&buffer[..n], sender) // which must be send back to client
                .expect("Cannot echo back data to client");
        } else { // or is coming from a client
            sender = src;
            socket
                .send_to(&buffer[..n], arguments.forward_address)
                .expect("Cannot send data to forward server");
        }
    }
}

fn should_drop(probability: f32) -> bool {
    return rand::random::<f32>() < probability;
}