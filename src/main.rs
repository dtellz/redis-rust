#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("[+] Redis server listening on port 6379...");
    loop {
        match listener.accept() {
            Ok((_socket, addr)) => {
               println!("accepted new client: {:?}", addr);
               // let stream = TcpStream::connect(addr);
               let mut connection = _socket;
               connection.write(b"+PONG\r\n");
            },
            Err(e) => println!("couldn't accept client: {:?}", e),
       }
    }
}
