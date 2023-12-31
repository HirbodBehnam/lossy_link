use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct Arguments {
    pub bind_address: SocketAddr,
    pub forward_address: SocketAddr,
    pub loss_rate: f32,
    pub out_of_order_rate: f32,
}

/// Parses the command line arguments of the program.
///
/// Panics if they are wrong in any circumvention.
pub fn parse_arguments() -> Arguments {
    let mut arguments_iter = env::args();
    arguments_iter.next(); // skip the program name
    let bind_address = arguments_iter
        .next()
        .expect("Pass the bind address as the first argument to program");
    let bind_address = bind_address
        .to_socket_addrs()
        .expect("Cannot parse the bind address")
        .next()
        .expect("Cannot parse the bind address");
    let forward_address = arguments_iter
        .next()
        .expect("Pass the forward address as the second argument to program");
    let forward_address = forward_address
        .to_socket_addrs()
        .expect("Cannot parse the forward address")
        .next()
        .expect("Cannot parse the forward address");
    let loss_rate = arguments_iter
        .next()
        .map_or(0.1, |v| v.parse::<f32>().expect("Cannot parse loss rate"));
    if loss_rate > 1.0 || loss_rate < 0.0 {
        panic!("Loss rate must be between 0 and 1");
    }
    let out_of_order_rate = arguments_iter
        .next()
        .map_or(0.1, |v| v.parse::<f32>().expect("Cannot parse out of order rate"));
    if out_of_order_rate > 1.0 || out_of_order_rate < 0.0 {
        panic!("Out of order rate must be between 0 and 1");
    }
    return Arguments {
        bind_address,
        forward_address,
        loss_rate,
        out_of_order_rate,
    };
}
