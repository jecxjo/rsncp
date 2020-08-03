use crate::common;
use flate2::{Compression, GzBuilder};
use std::io::prelude::*;
use std::net::TcpStream;
use tar::Builder;

pub struct Send {
    destination: common::Destination,
    files: Vec<common::File>,
}

impl Send {
    pub fn new(destination: common::Destination, files: Vec<common::File>) -> Self {
        Send { destination, files }
    }

    pub fn do_send(&self) -> Result<(), String> {
        let dst = format!("{}:8002", self.destination);

        let mut ar = Builder::new(Vec::new());
        let compressed_data = Vec::new();

        let mut gz = GzBuilder::new().write(compressed_data, Compression::fast());

        println!(
            "[*] copying {:?} to ip : {}",
            self.files.as_slice(),
            self.destination
        );

        let data = common::validate_files(&self.files)?;
        println!("[#] start writing files");
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
        let mut stream =
            TcpStream::connect(&dst).or(Err(String::from("Could not connect to listener")))?;

        println!("[#] sending...");
        stream
            .write(sending_data)
            .or(Err(String::from("Failed to send data")))?;
        Ok(())
    }
}
