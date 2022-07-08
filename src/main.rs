use colored::*;
use integer_encoding::VarInt;
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub mod handshake;
pub mod hostname;
pub mod ip;
pub mod len_trait;

use handshake::Handshake;
use hostname::Hostname;
use ip::Ip;
use len_trait::VarIntLen;

fn perform_connection<T: VarIntLen + ToString>(
    h: &Handshake<T>,
    t: Duration,
    short: bool,
) -> Result<(), ()> {
    let bytes = h.to_byte_vec();

    match TcpStream::connect_timeout(&h.to_string().to_socket_addrs().unwrap().last().unwrap(), t) {
        Ok(mut stream) => {
            println!("[INFO] Connected to the server!");
            stream.write(&bytes).unwrap();
            let mut buffer = [0 as u8; 1024];
            stream
                .set_read_timeout(Some(t))
                .expect("Invalid read timeout");
            loop {
                let result = match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!("[INFO] Read {} bytes from the server!", size);
                        if short && size > 0 {
                            return Ok(());
                        }
                        if size > 5 {
                            println!("[INFO] Prefix: {:x?}", &buffer[0..5]);
                            match std::str::from_utf8(&buffer[5..size]) {
                                Ok(s) => println!("[INFO] {}", s),
                                Err(e) => println!("[ERR] {}", e),
                            }
                        } else {
                            println!("[WARN] Received less than 5 bytes!");
                        }
                        if size != 0 {
                            Ok(())
                        } else {
                            Err(())
                        }
                    }
                    Err(e) => {
                        println!("[ERR] Error reading from the server: {}", e);
                        Err(())
                    }
                };
                if result.is_err() {
                    break;
                }
            }
        }
        Err(e) => {
            println!("[ERR] Connection error: {}", e);
            return Err(());
        }
    }
    Ok(())
}

fn main() {
    println!("Running the MC script!");

    let short_circuit = true;

    let h = Handshake::new(Ip::new(141, 144, 198, 21), 25565);
    println!("[INFO] Trying {}", h.to_string());
    perform_connection(&h, Duration::from_millis(5000), false);

    // for i in 1..=255 {
    //     let h = Handshake::new(Ip::new(141, 144, 198, i), 25565);
    //     println!("[INFO] Trying {}", h.to_string());
    //     match perform_connection(&h, Duration::from_millis(100), short_circuit) {
    //         Ok(_) => println!("{}", format!("[INFO] Success on {}!", i).green()),
    //         Err(_) =>  println!("{}", format!("[INFO] Failed on {}!", i).red()),
    //     }
    // }

    println!("[INFO] Finished!");
}
