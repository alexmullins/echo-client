use std::env;
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() {
    // get (hostname or ip) and ([port] or 7)
    let hostnameport = match env::args().nth(1) {
        Some(h) => match env::args().nth(2) {
            Some(p) => format!("{}:{}", h, p),
            None => format!("{}:{}", h, 7),
        },
        None => {
            println!("[ping-client]: must provide a hostname or IP address to connect to.");
            return;
        }
    };

    // resolve the hostnameport to a socket addr
    let socket_addr = match hostnameport.to_socket_addrs().unwrap().next() {
        Some(sock) => sock,
        None => {
            println!(
                "[ping-client]: err: couldn't resolve hostname {}",
                hostnameport
            );
            return;
        }
    };

    // connect the socket addr with a timeout
    let timeout = Duration::from_secs(3);
    let stream = match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(s) => s,
        Err(err) => {
            println!("[ping-client]: err: {}", err);
            return;
        }
    };

    // try writing a string
    // read back a string and compare to above

    // get peer's address (IPv4 or IPv6)
    let ipaddr = match stream.peer_addr() {
        Ok(ip) => ip,
        Err(err) => {
            println!("[ping-client]: err: {}", err);
            return;
        }
    };

    // Successfully connected
    println!("Connected to {} [{}]", hostnameport, ipaddr);

    // Now close connection
    stream
        .shutdown(Shutdown::Both)
        .expect("[ping-client]: couldn't close connection");

    return;
}
