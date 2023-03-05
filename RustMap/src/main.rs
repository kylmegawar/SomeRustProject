use std::env::args;
use std::net::{TcpStream, UdpSocket};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = args().collect();
    let ip_address = &args[1];
    let protocol = &args[2];
    for arg in args.iter(){
        println!("{}",arg);
    }

    if protocol == "udp" {
        for n in 1..50 {
            let port = n;
            let address = format!("{}:{}", ip_address, port);

                let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
                socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
                let buf = [0u8; 1];
                match socket.send_to(&buf, address) {
                    Ok(_) => println!("port {} open", port),
                    Err(_) => print!(""),
                }



        }
    } else if protocol == "tcp" {
        for n in 1..65535 {
            let port = n;
            let address = format!("{}:{}", ip_address, port);

            let handle = thread::spawn(move || {
                match TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(1)) {
                    Ok(_) => println!("port {} open", port),
                    Err(_) => (),
                }
            });
        }
    }
    else {
        println!("Unknown protocol: {}", protocol);
    }
}
