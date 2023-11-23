use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

use log::{debug, info, warn};

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
    let mut sender = SocketAddr::from_str("127.0.0.1:0").unwrap(); // dummy value at first
    let mut packet_number: u32 = 0; // count packets for nicer logs
    let socket = UdpSocket::bind(arguments.bind_address).expect("Cannot listen on UDP socket");
    loop {
        let (n, src) = socket.recv_from(&mut buffer).expect("Cannot receive data");
        // Log this packet
        packet_number += 1;
        debug!("Got packet {} from {} with size {}", packet_number, src, n);
        // Maybe drop this packet?
        if should_drop(arguments.loss_rate) {
            warn!("Dropping packet {}", packet_number);
            continue;
        }
        // Two possibilities...
        if src == arguments.forward_address {
            // either this packet is coming from server...
            socket
                .send_to(&buffer[..n], sender) // which must be send back to client
                .expect("Cannot echo back data to client");
        } else {
            // or is coming from a client
            sender = src; // set the current client so packets coming back from server get forwarded to client
            info!("Client is now {}", sender);
            socket
                .send_to(&buffer[..n], arguments.forward_address) // and send it to forwarded server
                .expect("Cannot send data to forward server");
        }
    }
}

/// Returns true if the packet should be dropped
fn should_drop(probability: f32) -> bool {
    return rand::random::<f32>() < probability;
}
