use std::io::prelude::*;
use std::net::TcpListener;
use std::env::args;
use std::net::{TcpStream, UdpSocket};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1337")?;
    let clients = Arc::new(Mutex::new(vec![]));

    loop {
        let (stream, _) = listener.accept()?;
        let clients_ref = clients.clone();

        let _handle = thread::spawn(move || {
            let mut client = stream.try_clone().unwrap();
            let client_addr = client.peer_addr().unwrap();
            println!("Connected to: {}", client_addr);

            {
                let mut clients = clients_ref.lock().unwrap();
                clients.push(client.try_clone().unwrap());
                println!("Clients are {:?}", clients);
            }

            loop {
                let mut buffer = [0; 1024];

                match client.read(&mut buffer) {
                    Ok(0) => {
                        println!("Client {} disconnected", client_addr);
                        let mut clients = clients_ref.lock().unwrap();
                        clients.retain(|c| c.peer_addr().unwrap() != client_addr);
                        break;
                    }
                    Ok(n) => {
                        let message = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                        println!("Received from {}: {}", client_addr, message);

                        let clients = clients_ref.lock().unwrap();
                        for mut c in clients.iter() {
                            if c.peer_addr().unwrap() != client_addr {
                                let msg = format!("{}: {}", client_addr, message);
                                c.write(msg.as_bytes()).unwrap();
                            }
                        }
                    }
                    Err(_) => {
                        println!("Error reading from client {}", client_addr);
                        break;
                    }
                }
            }
        });
    }
}
