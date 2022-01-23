use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#CPX
pub struct CPX {
    mode: AddressingMode
}

impl CPX {
    pub fn new(mode: AddressingMode) -> Self {
        CPX{ mode }
    }
}

impl Display for CPX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPX {}", self.mode)
    }
}

impl Instruction for CPX {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let mem = cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Carry, cpu.index_register_x >= mem);
        cpu.set_flag(StatusFlag::Zero, cpu.index_register_x == mem);
        cpu.set_flag(StatusFlag::Negative, cpu.index_register_x.wrapping_sub(mem) > 0x7F);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::Immediate(value) => vec![0xE0, value],
            AddressingMode::ZeroPage(addr) => vec![0xE4, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0xEC, addr),
            _ => panic!("Addressing mode not allowed for CPX")
        }
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        format!("CPX {}", self.mode.debug_string(&cpu))
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode::ZeroPage, AddressingMode::Immediate, Instruction, AddressingMode};
    use super::CPX;

    #[test]
    fn values_are_equal_positive() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x3B;

        // When
        CPX::new(Immediate(0x3B)).execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0011, cpu.processor_status);
    }

    #[test]
    fn acc_greater_and_negative() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0xAB;

        // When
        CPX::new(Immediate(0x0B)).execute(&mut cpu);

        // Then
        assert_eq!(0b1000_0001, cpu.processor_status);
    }

    #[test]
    fn acc_less_and_positive() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x02;

        // When
        let mode = ZeroPage(0x88);
        cpu.write(&mode, 0xAF);
        CPX::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0, cpu.processor_status);
    }

    #[test]
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x80;
        cpu.index_register_x = 0x40;
        cpu.index_register_y = 0x80;
        cpu.processor_status = 0x25;

        // When
        CPX::new(Immediate(0x41)).execute(&mut cpu);

        // Then
        assert_eq!(0x80, cpu.accumulator);
        assert_eq!(0x40, cpu.index_register_x);
        assert_eq!(0x80, cpu.index_register_y);
        assert_eq!(0xA4, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let cpx = CPX::new(ZeroPage(0xC0));

        assert_eq!("CPX $C0", cpx.to_string())
    }

    #[test]
    fn bytes_representation() {
        let cpx = CPX::new(ZeroPage(0x0A));

        assert_eq!(vec![0xE4, 0x0A], cpx.bytes());
    }

    #[test]
    fn bytes_representation_absolute() {
        let cpx = CPX::new(AddressingMode::Absolute(0xAB92));

        assert_eq!(vec![0xEC, 0x92, 0xAB], cpx.bytes());
    }
}
