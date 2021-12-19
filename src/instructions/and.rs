use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#AND
pub(super) struct AND {
    opcode: u8,
    mode: AddressingMode
}

impl AND {
    pub fn new(opcode: u8, mode: AddressingMode) -> Self {
        AND{ opcode, mode }
    }
}

impl Display for AND {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AND {}", self.mode)
    }
}

impl Instruction for AND {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.accumulator &= cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        let mut byte_vec = vec![self.opcode];
        byte_vec.extend(self.mode.bytes());

        return byte_vec;
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate};
    use crate::instructions::and::AND;

    #[test]
    fn basic_and() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xD3);

        cpu.accumulator = 0b01101110;
        cpu.write(&mode, 0b00110111);

        // When
        AND::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0b00100110);
        assert_eq!(cpu.processor_status, 0);
    }

    #[test]
    fn zero_flag_and() {
        // Given
        let mut cpu = CPU::empty();
        let mode = Immediate(0);

        cpu.accumulator = 0b01101110;

        // When
        AND::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.processor_status, 0b00000010);  // Zero flag is set
    }

    #[test]
    fn negative_flag_and() {
        // Given
        let mut cpu = CPU::empty();
        let mode = Immediate(0xF0);

        cpu.accumulator = 0b11011110;

        // When
        AND::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0b11010000);
        assert_eq!(cpu.processor_status, 0b10000000);  // Negative flag is set
    }

    #[test]
    fn string_representation() {
        // Given
        let and = AND::new(Immediate(0x8C));

        // Then
        assert_eq!("AND #$8C", and.to_string())
    }
}
