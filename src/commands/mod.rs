pub mod send;
pub mod listen;

use crate::common::{Destination, File};
use send::Send;


pub fn do_send(destination: Destination, files: Vec<File>) {
    // Convert directories in files to files

    let send = Send::new(destination, files);

    match send.do_send() {
        Ok(_) => println!("done"),
        Err(e) => println!("ERROR: {}", e),
    }
}
