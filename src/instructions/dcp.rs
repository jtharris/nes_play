use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};

// https://www.masswerk.at/6502/6502_instruction_set.html#DCP
pub(super) struct DCP {
    mode: AddressingMode
}

impl DCP {
    pub fn new(mode: AddressingMode) -> Self {
        DCP { mode }
    }
}

impl Display for DCP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*DCP {}", self.mode)
    }
}

impl Instruction for DCP {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let (val, _) = cpu.read(&self.mode).overflowing_sub(1);
        cpu.write(&self.mode, val);

        cpu.set_flag(StatusFlag::Carry, cpu.accumulator >= val);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == val);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator.wrapping_sub(val) > 0x7F);

        cpu.memory_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::AddressingMode::{IndirectX, ZeroPage};
    use crate::cpu::{AddressingMode, CPU, Instruction};
    use crate::instructions::dcp::DCP;

    #[test]
    fn zero_flag_set() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0x88);

        cpu.write(&mode, 0x2A);
        cpu.accumulator = 0x29;

        // When
        DCP::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x29, cpu.read(&ZeroPage(0x88)));
        assert_eq!(0b0000_0011, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let dcp = DCP::new(IndirectX(0xAA));

        assert_eq!("*DCP ($AA,X)", dcp.to_string());
    }
}