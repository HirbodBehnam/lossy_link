use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct Arguments {
    pub bind_address: SocketAddr,
    pub forward_address: SocketAddr,
    pub loss_rate: f32,
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
        .map_or(0.1, |v| v.parse::<f32>().expect("cannot parse loss rate"));
    return Arguments {
        bind_address,
        forward_address,
        loss_rate,
    };
}
