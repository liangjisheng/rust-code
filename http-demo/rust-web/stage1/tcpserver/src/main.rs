use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on port 8080");
    for stream in listener.incoming() {
        let mut _stream = stream.unwrap();
        println!("connection established.");
        let mut buffer = [0; 1024];
        _stream.read(&mut buffer).unwrap();
        _stream.write(&mut buffer).unwrap();
    }
}
