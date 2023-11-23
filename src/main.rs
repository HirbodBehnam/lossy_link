use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    sync::mpsc::{channel, Receiver},
    thread,
    time::Duration,
};

use log::{debug, info, warn};

mod argument;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let arguments = argument::parse_arguments();
    info!(
        "Packet loss rate is configured to be {}",
        arguments.loss_rate
    );
    info!(
        "Out of order rate is configured to be {}",
        arguments.out_of_order_rate
    );
    info!("Listening on {}", arguments.bind_address);
    info!("Forwarding to {}", arguments.forward_address);
    // Start the listener
    let socket = UdpSocket::bind(arguments.bind_address).expect("Cannot listen on UDP socket");
    // Create the thread which sends the data from client to server
    let (client_server_tx, client_server_rx) = channel();
    let cloned_socket = socket.try_clone().unwrap();
    thread::spawn(move || {
        start_out_of_order_channel(
            cloned_socket,
            client_server_rx,
            arguments.out_of_order_rate,
            arguments.forward_address,
        );
    });
    // Start the loop to send data
    let mut packet_number: u32 = 0; // count packets for nicer logs
    let mut buffer = [0; 4 * 1024];
    // What is the client's IP and port which is connected to this program?
    // The initial value is a dummy value.
    let mut sender = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    loop {
        let (n, src) = socket.recv_from(&mut buffer).expect("Cannot receive data");
        // Log this packet
        packet_number += 1;
        debug!("Got packet {} from {} with size {}", packet_number, src, n);
        // Maybe drop this packet?
        if biased_bool(arguments.loss_rate) {
            warn!("Dropping packet {}", packet_number);
            continue;
        }
        // Two possibilities...
        if src == arguments.forward_address {
            // either this packet is coming from server...
            socket
                .send_to(&buffer[..n], sender) // which must be send back to client
                .expect("Cannot echo back data to client");
            // We also don't do out of order from server to client
        } else {
            // or is coming from a client
            sender = src; // set the current client so packets coming back from server get forwarded to client
            info!("Client is now {}", sender);
            // Send to channel
            client_server_tx
                .send((buffer[..n].to_owned(), packet_number))
                .unwrap();
        }
    }
}

/// Starts an out of order channel which sends the packets received in a channel
/// out of order based on a probability.
fn start_out_of_order_channel(
    socket: UdpSocket,
    recv_channel: Receiver<(Vec<u8>, u32)>,
    out_of_order_rate: f32,
    forward_address: SocketAddr,
) {
    // Wait for packets...
    while let Ok((packet, packet_id)) = recv_channel.recv() {
        if biased_bool(out_of_order_rate) {
            // If we reach here, We shall check if there is another packet waiting or not.
            // We also give the OS scheduler a chance to go and check the main thread.
            thread::sleep(Duration::from_millis(20));
            // Now check the channel again
            if let Ok((packet2, packet_id2)) = recv_channel.try_recv() {
                // Boom! Let's go out of order
                warn!("Out of order in packets {} and {}", packet_id, packet_id2);
                socket
                    .send_to(&packet2, forward_address) // and send it to forwarded server
                    .expect("Cannot send data to forward server");
                socket
                    .send_to(&packet, forward_address) // and send it to forwarded server
                    .expect("Cannot send data to forward server");
                continue; // forget the next branch
            }
        }
        // Just send the packet in order
        socket
            .send_to(&packet, forward_address) // and send it to forwarded server
            .expect("Cannot send data to forward server");
    }
}

/// Returns true with given probability
fn biased_bool(probability: f32) -> bool {
    return rand::random::<f32>() < probability;
}
