use crate::common;

pub struct Send {
    destination: common::Destination,
    files: Vec<common::File>,
}

impl Send {
    pub fn new(destination: common::Destination, files: Vec<common::File>) -> Self {
        Send {
            destination,
            files
        }
    }

    pub fn do_send(&self) -> Result<(), String> {
        match common::validate_files(&self.files) {
            Err(e) => Err(e),
            Ok(files) => {
                for f in files.iter() {
                    println!("* {}", f);
                }
                Ok(())
            },
        }
    }
}
