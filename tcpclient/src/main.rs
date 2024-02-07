use std::net::TcpStream;

fn main() {
    let _stream = TcpStream::connect("127.0.0.1:3030").unwrap();
    print!("连接成功")
}
