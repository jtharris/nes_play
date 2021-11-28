use std::fs;
use std::convert::TryInto;
use std::fs::File;
use crate::rom::{INes2Header, INesRom};
use crate::instructions::factory::generate_instruction;

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
        let rom = INesRom::new(contents);

        println!("{}", rom.header);
    }
}

pub struct Log {
    rom_filename: String,
    log_filename: String
}

impl Log {
    pub fn new(rom_file: &str, log_file: &str) -> Self {
        Log {
            rom_filename: rom_file.parse().unwrap(),
            log_filename: log_file.parse().unwrap()
        }
    }
}

impl Command for Log {
    fn execute(&self) {
        let contents = fs::read(&self.rom_filename).expect("Could not read file");
        let rom = INesRom::new(contents);

        let mut cpu = rom.to_cpu();
        // TODO:  Support Stdout if filename is missing?
        let mut log = File::create(&self.log_filename);

        match log {
            Result::Ok(l) => cpu.log_execution(Box::new(l)),
            Result::Err(e) => eprint!("{}", e)
        }
    }
}