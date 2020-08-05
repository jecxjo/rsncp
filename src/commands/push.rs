use crate::common;
use crate::common::networking;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::{thread, time};

pub struct Push {
    legacy: bool,
    files: Vec<common::File>,
}

impl Push {
    pub fn new(legacy: bool, files: Vec<common::File>) -> Self {
        Push {
            legacy,
            files,
        }
    }

    pub fn do_push(&self) -> Result<(), String> {
        let announce = format!("Multicasting for rsncp version {}", common::version());
        let announce = announce.as_bytes();

        let compressed = common::compression::pack(self.legacy, &self.files)?;
        let sending_data: &[u8] = &compressed;

        println!("[*] starting X-Casting, waiting for TCP connect");
        let addr = SocketAddr::new(*networking::IPV4, networking::PORT);
        let broadcast_socket =
            networking::new_sender(&addr).or(Err(String::from("Failed to create new sender")))?;
        let listener =
            TcpListener::bind("0.0.0.0:8002").or(Err(String::from("Failed to open listener")))?;
        listener
            .set_nonblocking(true)
            .or(Err(String::from("Failed to configure port")))?;
        let mut incoming_data;

        loop {
            broadcast_socket
                .send_to(announce, &addr)
                .or(Err(String::from("Failed to send announcement")))?;

            thread::sleep(time::Duration::from_millis(1000)); // Sleep half second for response

            match listener.accept() {
                Ok((mut socket, addr)) => {
                    incoming_data = [0u8; 1024];

                    match socket.read(&mut incoming_data) {
                        Ok(n) => {
                            let ack = std::str::from_utf8(&incoming_data[..n])
                                .or(Err(String::from("Ack not UTF-8")))?;
                            println!("[*] connection from {}", addr);
                            println!("[*] Client answer: {}", ack);
                            println!("[#] sending...");

                            socket
                                .write(sending_data)
                                .or(Err(String::from("Failed to send data")))?;

                            break;
                        }
                        Err(_) => println!("[?] Connection failed from {}", addr),
                    }
                }
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        println!(".");
                    } else {
                        return Err(String::from("Socket error"));
                    }
                }
            }
        }

        Ok(())
    }
}
