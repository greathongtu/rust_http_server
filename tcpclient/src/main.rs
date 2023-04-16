use std::io::{Read, Write};
use std::str;
use std::net::TcpStream;
fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello from client!".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();
    println!("Received: {}", str::from_utf8(&buffer).unwrap());
}
