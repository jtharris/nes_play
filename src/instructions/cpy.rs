use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#CPY
pub struct CPY {
    mode: AddressingMode
}

impl CPY {
    pub fn new(mode: AddressingMode) -> Self {
        CPY{ mode }
    }
}

impl Display for CPY {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPY {}", self.mode)
    }
}

impl Instruction for CPY {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let mem = cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Carry, cpu.index_register_y >= mem);
        cpu.set_flag(StatusFlag::Zero, cpu.index_register_y == mem);
        cpu.set_flag(StatusFlag::Negative, cpu.index_register_y.wrapping_sub(mem) > 0x7F);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::Immediate(value) => vec![0xC0, value],
            AddressingMode::ZeroPage(addr) => vec![0xC4, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0xCC, addr),
            _ => panic!("Addressing mode not allowed for CPY")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode::Absolute, AddressingMode::Immediate, Instruction};
    use crate::cpu::AddressingMode::ZeroPage;
    use super::CPY;

    #[test]
    fn values_are_equal_positive() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x3B;

        // When
        CPY::new(Immediate(0x3B)).execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0011, cpu.processor_status);
    }

    #[test]
    fn acc_greater_and_negative() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0xAB;

        // When
        CPY::new(Immediate(0x0B)).execute(&mut cpu);

        // Then
        assert_eq!(0b1000_0001, cpu.processor_status);
    }

    #[test]
    fn acc_less_and_positive() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x02;

        // When
        let mode = Absolute(0x0288);
        cpu.write(&mode, 0xAF);
        CPY::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0, cpu.processor_status);
    }

    #[test]
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x80;
        cpu.index_register_y = 0x40;
        cpu.processor_status = 0x25;

        // When
        CPY::new(Immediate(0x41)).execute(&mut cpu);

        // Then
        assert_eq!(0x80, cpu.accumulator);
        assert_eq!(0x40, cpu.index_register_y);
        assert_eq!(0xA4, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let cpy = CPY::new(Immediate(0xF9));

        assert_eq!("CPY #$F9", cpy.to_string())
    }

    #[test]
    fn bytes_representation() {
        let cpy = CPY::new(ZeroPage(0xAA));

        assert_eq!(vec![0xC4, 0xAA], cpy.bytes());
    }
}
