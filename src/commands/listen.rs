use flate2::write::GzDecoder;
use std::io::prelude::*;
use std::net::TcpListener;
use tar::Archive;

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

        let mut writer = Vec::new();
        let mut decoder = GzDecoder::new(writer);
        decoder
            .write_all(&data)
            .or(Err(String::from("Failed to decompress stream")))?;
        writer = decoder
            .finish()
            .or(Err(String::from("Issue with compression")))?;

        let mut ar = Archive::new(&writer[..]);
        ar.unpack(".")
            .or(Err(String::from("Failed to unpack tarbal")))?;

        Ok(())
    }
}
