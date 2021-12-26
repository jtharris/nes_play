use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};

// https://www.masswerk.at/6502/6502_instruction_set.html#RLA
pub(super) struct RLA {
    mode: AddressingMode
}

impl RLA {
    pub fn new(mode: AddressingMode) -> Self {
        RLA { mode }
    }
}

impl Display for RLA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*RLA {}", self.mode)
    }
}

impl Instruction for RLA {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let value = cpu.read(&self.mode);
        let new_carry = value > 0x7F;
        let mut shifted_value = value << 1;

        if cpu.get_flag(StatusFlag::Carry) {
            shifted_value |= 0x01;
        }

        cpu.write(&self.mode, shifted_value);

        cpu.accumulator &= shifted_value;
        cpu.set_flag(StatusFlag::Carry, new_carry);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        cpu.memory_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};
    use crate::instructions::rla::RLA;

    #[test]
    fn basic_behavior() {
        // Given
        let mut cpu = CPU::empty();

        cpu.accumulator = 0x16;
        cpu.set_flag(StatusFlag::Carry, true);
        cpu.write(&AddressingMode::ZeroPage(0xF9), 0x7F);

        // When
        RLA::new(AddressingMode::ZeroPage(0xF9)).execute(& mut cpu);

        // Then
        assert_eq!(0x16, cpu.accumulator);
        assert_eq!(0xFF, cpu.read(&AddressingMode::ZeroPage(0xF9)));
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let rla = RLA::new(AddressingMode::AbsoluteX(0x8AF3));

        assert_eq!("*RLA $8AF3,X", rla.to_string());
    }
}