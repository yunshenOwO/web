use std::io::{Read, Write};
use std::net::{TcpListener};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0;1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
        println!("接收倒一次");
    }
}