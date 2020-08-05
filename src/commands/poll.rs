use crate::common::{RX_RETRIES, compression, networking};
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::{thread, time};

pub struct Poll {}

impl Poll {
    pub fn do_poll(&self) -> Result<(), String> {
        let pusher;

        // Detection of multicast
        {
            let addr = SocketAddr::new(*networking::IPV4, networking::PORT);
            let listener = networking::join_multicast(addr)
                .or(Err(String::from("Failed to join multicast")))?;
            println!("[#] Joining multicast group");

            let mut buf = [0u8; 1024]; // receive buffer

            loop {
                println!("[*] waiting for something-cast");
                match listener.recv_from(&mut buf) {
                    Ok((len, remote_addr)) => {
                        let data = &buf[..len];
                        if remote_addr.port() == networking::PORT {
                            println!("[*] found pusher at {}", remote_addr);
                            let announcement = std::str::from_utf8(data)
                                .or(Err(String::from("Announcement not UTF-8")))?;
                            println!("[#] Announcement: {}", announcement);
                            pusher = remote_addr;
                            break;
                        } else {
                            println!("[?] received garbage from {}", remote_addr);
                        }
                    }
                    Err(_) => return Err(String::from("Error when receiving from multicast")),
                }
            }

            // closes socket
        }

        let mut stream =
            TcpStream::connect(pusher).or(Err(String::from("Failed to connect to pusher")))?;

        stream
            .write("I'm ready".as_bytes())
            .or(Err(String::from("Failed to ack announcement")))?;

        let mut data = Vec::new();
        let mut incoming_data;
        let mut no_rx_cnt = 0;

        loop {
            incoming_data = [0; 512];
            match stream.read(&mut incoming_data) {
                Ok(n) => {
                    let mut convert: Vec<u8> = incoming_data[..n].iter().cloned().collect();
                    data.append(&mut convert);

                    if n == 0 {
                        no_rx_cnt = no_rx_cnt + 1;

                        if no_rx_cnt > RX_RETRIES {
                            break;
                        } else {
                            thread::sleep(time::Duration::from_millis(100));
                        }
                    } else {
                        no_rx_cnt = 0;
                    }
                }
                _ => {}
            }
        }

        compression::unpack(&data)?;

        Ok(())
    }
}
