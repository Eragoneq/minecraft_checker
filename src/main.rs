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
use std::error::Error;
use std::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct SizeError;

impl std::fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid byte size!")
    }
}

impl Error for SizeError {}

fn perform_connection<T: VarIntLen + ToString>(
    h: &Handshake<T>,
    t: Duration,
    short: bool,
) -> Result<()> {
    let bytes = h.to_byte_vec();

    let mut stream =
        TcpStream::connect_timeout(&h.to_string().to_socket_addrs().unwrap().last().unwrap(), t)?;
    println!("[INFO] Connected to the server!");

    stream.write(&bytes).unwrap();
    let mut buffer = [0 as u8; 1024];
    stream
        .set_read_timeout(Some(t))
        .expect("Invalid read timeout");

    // let mut x = 0;

    loop {
        let size = stream.read(&mut buffer)?;
        println!("[INFO] Read {} bytes from the server!", size);
        // println!("{:?}", &buffer[0..5].decode_var());

        // x += size;
        if short && size > 0 {
            return Ok(());
        }

        if size > 5 {
            println!("[INFO] Prefix: {:x?}", &buffer[0..5]);
            let s = std::str::from_utf8(&buffer[5..size])?;
            println!("[INFO] {}", s);
        } else {
            println!("[WARN] Received less than 5 bytes!");
        }

        if size == 0 {
            return Err(SizeError.into());
        }
    }
}

fn main() {
    println!("Running the MC script!");

    let short_circuit = true;

    let h = Handshake::new(Ip::new(144,21,32,166), 25565);
    println!("[INFO] Trying {}", h.to_string());
    match perform_connection(&h, Duration::from_millis(5000), false) {
        Err(e) => println!("[ERR] {:?}", e),
        _ => (),
    }

    // for i in 1..=255 {
    //     let h = Handshake::new(Ip::new(144,21,32,i), 25565);
    //     println!("[INFO] Trying {}", h.to_string());
    //     match perform_connection(&h, Duration::from_millis(100), short_circuit) {
    //         Ok(_) => println!("{}", format!("[INFO] Success on {}!", i).green()),
    //         Err(_) =>  println!("{}", format!("[INFO] Failed on {}!", i).red()),
    //     }
    // }

    println!("[INFO] Finished!");
}
