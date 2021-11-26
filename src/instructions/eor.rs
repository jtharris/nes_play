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
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate, AbsoluteY};
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
}
