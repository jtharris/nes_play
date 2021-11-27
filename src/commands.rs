use std::fs;
use std::convert::TryInto;
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

pub struct Assembly {
    rom_filename: String
}

impl Assembly {
    pub fn new(file: &str) -> Self {
        Assembly { rom_filename: file.parse().unwrap() }
    }
}

impl Command for Assembly {
    fn execute(&self) {
        let contents = fs::read(&self.rom_filename).expect("Could not read file");
        let rom = INesRom::new(contents);

        let mut cpu = rom.to_cpu();
        while let Some(instruction) = generate_instruction(&mut cpu) {
            println!("{}", instruction)
        }
    }
}