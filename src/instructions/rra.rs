use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};

// https://www.masswerk.at/6502/6502_instruction_set.html#RRA
pub(super) struct RRA {
    mode:  AddressingMode
}

impl RRA {
    pub fn new(mode: AddressingMode) -> Self {
        RRA { mode }
    }
}

impl Display for RRA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*RRA {}", self.mode)
    }
}

impl Instruction for RRA {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let value = cpu.read(&self.mode);
        let new_carry = value & 0x01 == 0x01;
        let mut new_value = value >> 1;

        if cpu.get_flag(StatusFlag::Carry) {
            new_value |= 0x80;
        }

        let sum = new_value as u16 + new_carry as u16 + cpu.accumulator as u16;
        let carry = sum > 0xFF;
        // Looking at logic implemented here:
        // https://github.com/bfirsh/jsnes/blob/master/src/cpu.js
        let overflow = (cpu.accumulator ^ new_value) & 0x80 == 0 &&
            (cpu.accumulator ^ sum as u8) & 0x80 != 0;

        cpu.accumulator = sum as u8;

        cpu.write(&self.mode, new_value);
        cpu.set_flag(StatusFlag::Carry, carry);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Overflow, overflow);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        cpu.memory_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0x67, addr],
            AddressingMode::ZeroPageX(addr) => vec![0x77, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0x6F, addr),
            AddressingMode::AbsoluteX(addr) => self.bytes_for_opcode(0x7F, addr),
            AddressingMode::AbsoluteY(addr) => self.bytes_for_opcode(0x7B, addr),
            AddressingMode::IndirectX(addr) => vec![0x63, addr],
            AddressingMode::IndirectY(addr) => vec![0x73, addr],
            _ => panic!("Addressing mode not allowed for RRA")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{AddressingMode, CPU, Instruction};
    use crate::instructions::rra::RRA;

    #[test]
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xB2;
        cpu.index_register_x = 0x02;
        cpu.processor_status = 0xE4;

        cpu.write(&AddressingMode::ZeroPage(0x47), 0x47);
        cpu.write(&AddressingMode::ZeroPage(0x48), 0x06);
        cpu.write(&AddressingMode::Absolute(0x0647), 0xA5);

        // When
        RRA::new(AddressingMode::IndirectX(0x45)).execute(&mut cpu);

        // Then
        assert_eq!(0x05, cpu.accumulator);
        assert_eq!(0x25, cpu.processor_status);
        assert_eq!(0x52, cpu.read(&AddressingMode::Absolute(0x0647)));
    }

    #[test]
    fn string_representation() {
        let rra = RRA::new(AddressingMode::ZeroPageY(0xA6));

        assert_eq!("*RRA $A6,Y", rra.to_string());
    }

    #[test]
    fn bytes_representation() {
        let rra = RRA::new(AddressingMode::IndirectX(0xFD));

        assert_eq!(vec![0x63, 0xFD], rra.bytes());
    }
}