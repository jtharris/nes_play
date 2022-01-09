use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#EOR
pub(super) struct EOR {
    mode: AddressingMode
}

impl EOR {
    pub fn new(mode: AddressingMode) -> Self {
        EOR{ mode }
    }
}

impl Display for EOR {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EOR {}", self.mode)
    }
}

impl Instruction for EOR {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let memory_value= cpu.read(&self.mode);
        cpu.accumulator ^= memory_value;

        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::Immediate(value) => vec![0x49, value],
            AddressingMode::ZeroPage(addr) => vec![0x45, addr],
            AddressingMode::ZeroPageX(addr) => vec![0x55, addr],
            AddressingMode::Absolute(addr) => vec![0x4D, addr.to_le_bytes()[0], addr.to_le_bytes()[1]],
            AddressingMode::AbsoluteX(addr) => vec![0x5D, addr.to_le_bytes()[0], addr.to_le_bytes()[1]],
            AddressingMode::AbsoluteY(addr) => vec![0x59, addr.to_le_bytes()[0], addr.to_le_bytes()[1]],
            AddressingMode::IndirectX(addr) => vec![0x41, addr],
            AddressingMode::IndirectY(addr) => vec![0x51, addr],
            _ => panic!("Addressing mode not allowed for EOR")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate, AbsoluteX, AbsoluteY};
    use crate::instructions::eor::EOR;

    #[test]
    fn basic_eor() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xD3);

        cpu.accumulator =      0b01101110;
        cpu.write(&mode, 0b00110111);

        // When
        EOR::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0b01011001);
        assert_eq!(cpu.processor_status, 0);
    }

    #[test]
    fn zero_flag_eor() {
        // Given
        let mut cpu = CPU::empty();
        let mode = Immediate(0b01101110);

        cpu.accumulator = 0b01101110;

        // When
        EOR::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.processor_status, 0b00000010);  // Zero flag is set
    }

    #[test]
    fn negative_flag_eor() {
        // Given
        let mut cpu = CPU::empty();
        let mode = Immediate(0b01011111);

        cpu.accumulator = 0b11011110;

        // When
        EOR::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0b10000001);
        assert_eq!(cpu.processor_status, 0b10000000);  // Negative flag is set
    }

    #[test]
    fn string_representation() {
        let eor = EOR::new(AbsoluteY(0x02AA));

        assert_eq!("EOR $02AA,Y", eor.to_string())
    }

    #[test]
    fn bytes_representation() {
        let eor = EOR::new(AbsoluteX(0x02AA));

        assert_eq!(vec![0x5D, 0xAA, 0x02], eor.bytes());
    }
}
