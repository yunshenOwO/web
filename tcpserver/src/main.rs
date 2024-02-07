use std::net::{TcpListener};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("connection ......")
    }
}