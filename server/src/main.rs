//#![deny(warnings)]
extern crate resolve_dns;

use std::{net::{UdpSocket, SocketAddr}, process::exit};
use std::thread;

use resolve_dns::get_message;

fn handle_client(_: &SocketAddr, _: usize, buf: &[u8; 4096]) {
    let message = get_message(&Vec::from(buf.as_slice()));
    println!("{}", message.header.id)
}

fn main() {
    let socket: UdpSocket;
    let result = UdpSocket::bind("127.0.0.1:8888");
    match result{
        Ok(s) => { socket = s; },
        Err(e) =>  {
            println!("{e}");
            exit(1)
        }
    }

    loop {
        let mut buf = [0; 4096];
        let result = socket.recv_from(&mut buf);
        match result {
            Ok (s) => {
                thread::spawn(move || handle_client(&s.1, s.0, &buf));
            },
            Err(e) =>  {
                println!("{e}");
                exit(1)
            }
        } 
    }
}
