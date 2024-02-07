use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3030").unwrap();
    stream.write("hello".as_bytes()).unwrap();
    let mut buffer = [0;5];
    stream.read(&mut buffer).unwrap();
    println!("{:?}", str::from_utf8(&buffer).unwrap());
}
