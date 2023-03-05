use std::env;
use std::io::{self, prelude::*};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::process;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use magic_crypt::generic_array::typenum::U256;
use short_crypt::ShortCrypt;


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
                    let uncrypted_buffer = uncrypt(recv_buffer[..n].to_vec());
                    println!("{}", String::from_utf8_lossy(&uncrypted_buffer).trim());
                    println!("j'écoute");
                }
                _ => {
                    break;
                }
            }
        }
    });
    loop {
        reader.read_line(&mut input)?;
        let b64 = input_to_b64(input.trim().to_string());
        /*stream.write_all(input.as_bytes())?;
        input.clear();*/
        stream.write_all(b64.as_bytes())?;
        input.clear();

        let uncrypted_buffer = uncrypt(b64.as_bytes().to_vec());
        println!("{}", String::from_utf8_lossy(&uncrypted_buffer).trim());
    }
}

fn input_to_b64(input: String) -> String {
    let mut input = base64::encode(input);
    let args_vec: Vec<String> = env::args().collect();
    let key = args_vec[1].clone();
    println!("key: {}", key);
    let sc = ShortCrypt::new(&key);

    let mut input = sc.encrypt_to_url_component(&input);
    println!("input: {}", input);
    input
}

fn uncrypt(recv_buffer: Vec<u8>) -> Vec<u8> {
    let args_vec: Vec<String> = env::args().collect();
    let key = args_vec[1].clone();
    println!("key: {}", key);
    let sc = ShortCrypt::new(&key);
    let decrypted_buffer_str = String::from_utf8_lossy(&recv_buffer[..]);
    let mut decrypted_buffer = sc.decrypt_url_component(&decrypted_buffer_str).expect("Erreur lors du déchiffrement");
    let mut decrypted_buffer_str = String::from_utf8_lossy(&decrypted_buffer[..]).trim().to_string();
    let mut decoded_buffer = base64::decode(&decrypted_buffer).expect("Erreur lors du décodage");
    decoded_buffer
}