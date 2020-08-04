use crate::common;
use std::io::prelude::*;
use std::net::TcpStream;

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

        println!(
            "[*] copying {:?} to ip : {}",
            self.files.as_slice(),
            self.destination
        );

        let compressed = common::compression::pack(&self.files)?;
        let sending_data: &[u8] = &compressed;
        let mut stream =
            TcpStream::connect(&dst).or(Err(String::from("Could not connect to listener")))?;

        println!("[#] sending...");
        stream
            .write(sending_data)
            .or(Err(String::from("Failed to send data")))?;
        Ok(())
    }
}
