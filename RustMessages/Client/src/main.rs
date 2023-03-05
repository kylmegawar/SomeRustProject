use std::env::args;
use std::io::{self, prelude::*};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;




fn main() -> io::Result<()> {
    let ip_address = "127.0.0.1:1337";

    let mut stream = TcpStream::connect(&ip_address)?;

    let mut input = String::new();
    let mut buffer = [0; 1024];
    let mut reader = io::stdin();

    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");





    thread::spawn(move || {
        let mut recv_buffer = [0; 1024];
        loop {
            match stream_clone.read(&mut recv_buffer) {
                Ok(n) if n > 0 => {
                    println!("{}", String::from_utf8_lossy(&recv_buffer[..n]).trim());
                }
                _ => {
                    break;
                }
            }
        }
    });

    loop {
        reader.read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;
        input.clear();
    }
}
