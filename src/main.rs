extern crate failure;

use std::env;
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::time::Duration;

use failure::{format_err, Error};

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
    let socket_addr = match hostnameport.to_socket_addrs()?.next() {
        Some(sock) => sock,
        None => {
            return Err(format_err!(
                "[ping-client]: err: couldn't resolve hostname {}",
                hostnameport
            ));
        }
    };

    // connect the socket addr with a timeout
    let timeout = Duration::from_secs(3);
    let stream = match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(s) => s,
        Err(err) => {
            return Err(format_err!("[ping-client]: err: {}", err));
        }
    };

    // try writing a string
    // read back a string and compare to above

    // get peer's address (IPv4 or IPv6)
    let ipaddr = match stream.peer_addr() {
        Ok(ip) => ip,
        Err(err) => {
            return Err(format_err!("[ping-client]: err: {}", err));
        }
    };

    // Successfully connected
    println!("Connected to {} [{}]", hostnameport, ipaddr);

    // Now close connection
    stream
        .shutdown(Shutdown::Both)
        .expect("[ping-client]: couldn't close connection");

    return Ok(());
}
