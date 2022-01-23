use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#NOP
pub struct NOP {}

impl Display for NOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NOP")
    }
}

impl Instruction for NOP {
    fn execute(&self, _cpu: &mut CPU) -> u8 { 2 }

    fn bytes(&self) -> Vec<u8> {
        vec![0xEA]
    }
}


// https://www.masswerk.at/6502/6502_instruction_set.html#NOPs
pub struct IllegalNOP {
    opcode: u8,
    mode: Option<AddressingMode>
}

impl IllegalNOP {
    pub fn new(opcode: u8, mode: Option<AddressingMode>) -> Self {
        IllegalNOP{ opcode, mode }
    }
}

impl Display for IllegalNOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.mode {
            Some(m) => write!(f, "*NOP {}", m),
            None => write!(f, "*NOP")
        }
    }
}

impl Instruction for IllegalNOP {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        match &self.mode {
            Some(m) => cpu.default_cycles(m),
            None => 2
        }
    }

    fn bytes(&self) -> Vec<u8> {
        match &self.mode {
            &Some(AddressingMode::Immediate(addr)) |
            &Some(AddressingMode::ZeroPage(addr)) |
            &Some(AddressingMode::ZeroPageX(addr)) => vec![self.opcode, addr],
            &Some(AddressingMode::Absolute(addr)) |
            &Some(AddressingMode::AbsoluteX(addr)) => self.bytes_for_opcode(self.opcode, addr),
            None => vec![self.opcode],
            _ => panic!("Addressing mode not allowed for *NOP")
        }
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        match &self.mode {
            Some(m) => format!("*NOP {}", m.debug_string(&cpu)),
            None => self.to_string()
        }
    }
}
