use crate::cpu::{CPU, Instruction, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#CMP
pub struct CMP {
    mode: AddressingMode
}

impl CMP {
    pub fn new(mode: AddressingMode) -> Self {
        CMP{ mode }
    }
}

impl Instruction for CMP {
    fn execute(&self, cpu: &mut CPU) {
        let mem = cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Carry, cpu.accumulator >= mem);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == mem);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode::Immediate, Instruction};
    use super::CMP;
    use crate::cpu::AddressingMode::AbsoluteX;

    #[test]
    fn values_are_equal_positive() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0x3B;

        // When
        CMP::new(Immediate(0x3B)).execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0011, cpu.processor_status);
    }

    #[test]
    fn acc_greater_and_negative() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0xAB;

        // When
        CMP::new(Immediate(0x3B)).execute(&mut cpu);

        // Then
        assert_eq!(0b1000_0001, cpu.processor_status);
    }

    #[test]
    fn acc_less_and_positive() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0x02;

        // When
        let mode = AbsoluteX(0x88);
        cpu.write(&mode, 0x6F);
        CMP::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0, cpu.processor_status);
    }
}
