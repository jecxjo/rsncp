use crate::common::{compression, networking};
use std::io::prelude::*;
use std::net::TcpListener;

pub struct Listen {}

impl Listen {
    pub fn do_listen(&self) -> Result<(), String> {
        println!("[*] waiting for a connection");

        let mut listener = match TcpListener::bind(networking::listener_ipv6()) {
            Ok(listener) => {
                println!("[!] IPv4 & IPv6 Support");
                Some(listener)
            }
            Err(_) => None,
        };

        if listener.is_none() {
            listener = match TcpListener::bind(networking::listener_ipv4()) {
                Ok(listener) => {
                    println!("[!] IPv4 Only Support");
                    Some(listener)
                }
                _ => None,
            }
        }

        if let Some(listener) = listener {
            let (mut socket, addr) = listener
                .accept()
                .or(Err(String::from("Failed to connect socket")))?;

            println!("[*] connection from {}", addr);

            let mut data = Vec::new();
            let mut incoming_data;

            loop {
                incoming_data = [0; 512];
                match socket.read(&mut incoming_data) {
                    Ok(n) => {
                        let mut convert: Vec<u8> = incoming_data[..n].iter().cloned().collect();
                        data.append(&mut convert);

                        if n < 512 {
                            break;
                        }
                    }
                    _ => {}
                }
            }

            compression::unpack(&data)?;

            Ok(())
        } else {
            Err(String::from("No network support"))
        }
    }
}
