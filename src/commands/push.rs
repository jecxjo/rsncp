use crate::common;

pub struct Push {
    files: Vec<common::File>,
}

impl Push {
    pub fn new(files: Vec<common::File>) -> Self {
        Push { files }
    }

    pub fn do_push(&self) -> Result<(), String> {
        Err(String::from("Not supported"))
    }
}
