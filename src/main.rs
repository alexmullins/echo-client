extern crate failure;

use failure::{format_err, Error};

use std::env;
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() -> Result<(), Error> {
    // create host:port string
    let hostnameport = match env::args().nth(1) {
        Some(h) => match env::args().nth(2) {
            Some(p) => format!("{}:{}", h, p), // "host:port"
            None => format!("{}:{}", h, 7),    // "host:7"
        },
        None => {
            return Err(format_err!(
                "[ping-client]: must provide a hostname or IP address to connect to."
            ));
        }
    };
    // resolve the hostnameport to a socket addr
    let socket_addr = hostnameport
        .to_socket_addrs()?
        .nth(0)
        .ok_or(format_err!("[ping-client]: couldn't get IP address"))?;
    // connect to the socket addr with a timeout
    let stream = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3))?;
    // get peer's address (IPv4 or IPv6)
    let ipaddr = stream.peer_addr()?;
    // Successfully connected
    println!("Connected to {} @ {}", hostnameport, ipaddr);
    // Now close connection
    stream.shutdown(Shutdown::Both)?;
    return Ok(());
}
