use crate::common;
use crate::common::networking;
use flate2::{Compression, GzBuilder};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::{thread, time};
use tar::Builder;

pub struct Push {
    files: Vec<common::File>,
}

impl Push {
    pub fn new(files: Vec<common::File>) -> Self {
        Push { files }
    }

    pub fn do_push(&self) -> Result<(), String> {
        let announce = format!("Multicasting for rsncp version {}", common::version());
        let announce = announce.as_bytes();

        let mut ar = Builder::new(Vec::new());
        let compressed_data = Vec::new();
        let mut gz = GzBuilder::new().write(compressed_data, Compression::fast());

        println!("[*] Bundling {:?} ", self.files.as_slice());
        let data = common::validate_files(&self.files)?;
        println!("[#] start preparing files");
        for f in data.iter() {
            println!("[*] adding: {}", f);
            ar.append_path(f)
                .or(Err(String::from("Failed to read file")))?;
        }

        let data = ar
            .into_inner()
            .or(Err(String::from("Failed reading file archive")))?;
        gz.write_all(&data)
            .or(Err(String::from("Failed to compress")))?;
        let data = gz.finish().or(Err(String::from("Failed to compress")))?;
        let sending_data: &[u8] = &data;

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
