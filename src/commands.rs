use std::fs;
use crate::rom::INes2Header;
use std::convert::TryInto;

pub trait Command {
    fn execute(&self);
}

pub struct Info {
    // TODO:  See if it could be more efficient to hang onto the pointer...
    rom_filename: String
}

impl Info {
    pub fn new(file: &str) -> Self {
        Info { rom_filename: file.parse().unwrap() }
    }
}

impl Command for Info {
    fn execute(&self) {
        let contents = fs::read(&self.rom_filename).expect("Could not read file");
        let header_bytes = contents[0..16].try_into().expect("Header not found");

        let header = INes2Header::new(header_bytes);
        println!("{}", header);
    }
}