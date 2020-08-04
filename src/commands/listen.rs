use crate::common;
use std::io::prelude::*;
use std::net::TcpListener;

pub struct Listen {}

impl Listen {
    pub fn do_listen(&self) -> Result<(), String> {
        println!("[*] waiting for a connection");

        let listener =
            TcpListener::bind("0.0.0.0:8002").or(Err(String::from("Failed to open socket")))?;

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

        common::compression::unpack(&data)?;

        Ok(())
    }
}
