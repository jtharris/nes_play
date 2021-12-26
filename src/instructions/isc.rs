use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};

// https://www.masswerk.at/6502/6502_instruction_set.html#ISC
pub(super) struct ISC {
    mode: AddressingMode
}

impl ISC {
    pub fn new(mode: AddressingMode) -> Self {
        ISC { mode }
    }
}

impl Display for ISC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*ISC {}", self.mode)
    }
}

impl Instruction for ISC {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let (val, _) = cpu.read(&self.mode).overflowing_add(1);
        cpu.write(&self.mode, val);

        let memory_value= val as u16;
        let existing_carry = (cpu.processor_status & 0x01) as u16;

        let diff = (cpu.accumulator as u16)
            .wrapping_sub(memory_value)
            .wrapping_sub(1 - existing_carry);

        let carry = diff <= 0x00FF;
        // See logic in:  https://github.com/bfirsh/jsnes/blob/master/src/cpu.js
        let overflow = (cpu.accumulator ^ val) & 0x80 != 0 &&
            (cpu.accumulator ^ diff as u8) & 0x80 != 0;

        cpu.accumulator = diff as u8;
        cpu.set_flag(StatusFlag::Carry, carry);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Overflow, overflow);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        match &self.mode {
            AddressingMode::IndirectY(_) => 4,
            m => cpu.memory_cycles(m)
        }
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};
    use crate::instructions::isc::ISC;

    #[test]
    fn both_instructions_executed() {
        // Given
        let mut cpu = CPU::empty();

        cpu.write(&AddressingMode::ZeroPage(0xA2), 0x88);
        cpu.accumulator = 0x98;
        cpu.set_flag(StatusFlag::Carry, true);

        // When
        ISC::new(AddressingMode::ZeroPage(0xA2)).execute(&mut cpu);

        // Then
        assert_eq!(0x89, cpu.read(&AddressingMode::ZeroPage(0xA2)));
        assert_eq!(0x0F, cpu.accumulator);
        assert_eq!(0b0000_0001, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let isc = ISC::new(AddressingMode::ZeroPageX(0xFA));

        assert_eq!("*ISC $FA,X", isc.to_string());
    }
}