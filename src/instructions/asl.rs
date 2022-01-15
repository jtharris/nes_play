use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#ASL
pub(super) struct ASL {
    mode: AddressingMode
}

impl ASL {
    pub fn new(mode: AddressingMode) -> Self {
        ASL{ mode }
    }
}

impl Display for ASL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ASL {}", self.mode)
    }
}

impl Instruction for ASL {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let mem_value = cpu.read(&self.mode);
        let shifted_value = mem_value << 1;

        cpu.write(&self.mode, shifted_value);
        cpu.set_flag(StatusFlag::Carry, mem_value > 0x7F);
        cpu.set_flag(StatusFlag::Zero, shifted_value == 0);
        cpu.set_flag(StatusFlag::Negative, shifted_value > 0x7F);

        cpu.memory_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::Accumulator => vec![0x0A],
            AddressingMode::ZeroPage(addr) => vec![0x06, addr],
            AddressingMode::ZeroPageX(addr) => vec![0x16, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0x0E, addr),
            AddressingMode::AbsoluteX(addr) => self.bytes_for_opcode(0x1E, addr),
            _ => panic!("Addressing mode not allowed for ASL")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::AddressingMode::{Accumulator, ZeroPage, ZeroPageX};
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::asl::ASL;

    #[test]
    fn basic_acc_asl() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0b00111010;

        // When
        let asl = ASL::new(Accumulator);
        asl.execute(&mut cpu);

        // Then
        assert_eq!(0b01110100, cpu.accumulator);
        assert_eq!(0b00000000, cpu.processor_status);
    }

    #[test]
    fn mem_asl_with_zero_and_carry() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xE6);
        cpu.write(&mode, 0b10000000);

        // When
        let asl = ASL::new(mode);
        asl.execute(&mut cpu);

        // Then
        let mode = ZeroPage(0xE6);
        assert_eq!(0, cpu.read(&mode));
        assert_eq!(0b00000011, cpu.processor_status);  // carry and zero set
    }

    #[test]
    fn mem_asl_with_negative() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xCC);
        cpu.write(&mode, 0b01001100);

        // When
        let asl = ASL::new(mode);
        asl.execute(&mut cpu);

        // Then
        let mode = ZeroPage(0xCC);
        assert_eq!(0b10011000, cpu.read(&mode));
        assert_eq!(0b10000000, cpu.processor_status);  // negative set
    }

    #[test]
    fn string_representation() {
        // Given
        let asl = ASL::new(Accumulator);

        // Then
        assert_eq!("ASL A", asl.to_string())
    }

    #[test]
    fn bytes_representation() {
        // Given
        let asl = ASL::new(ZeroPageX(0xDD));

        // Then
        assert_eq!(vec![0x16, 0xDD], asl.bytes());

    }
}