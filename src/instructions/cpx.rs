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

impl Instruction for CPX {
    fn execute(&self, cpu: &mut CPU) {
        let mem = cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Carry, cpu.index_register_x >= mem);
        cpu.set_flag(StatusFlag::Zero, cpu.index_register_x == mem);
        cpu.set_flag(StatusFlag::Negative, cpu.index_register_x > 0x7F);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode::ZeroPage, AddressingMode::Immediate, Instruction};
    use super::CPX;

    #[test]
    fn values_are_equal_positive() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x3B;

        // When
        CPX::new(Immediate(0x3B)).execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0011, cpu.processor_status);
    }

    #[test]
    fn acc_greater_and_negative() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0xAB;

        // When
        CPX::new(Immediate(0x3B)).execute(&mut cpu);

        // Then
        assert_eq!(0b1000_0001, cpu.processor_status);
    }

    #[test]
    fn acc_less_and_positive() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x02;

        // When
        let mode = ZeroPage(0x88);
        cpu.write(&mode, 0x6F);
        CPX::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0, cpu.processor_status);
    }
}
