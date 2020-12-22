use crate::cpu::{CPU, Instruction, AddressingMode, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#LRS
pub struct LRS {
    mode: AddressingMode
}

impl LRS {
    pub fn new(mode: AddressingMode) -> Self {
        LRS{ mode }
    }
}

impl Instruction for LRS {
    fn execute(&self, cpu: &mut CPU) {
        let value = cpu.read(&self.mode);
        let carry = value & 0x01 == 0x01;
        let shifted = value >> 1;

        cpu.write(&self.mode, shifted);

        cpu.set_flag(StatusFlag::Carry, carry);
        cpu.set_flag(StatusFlag::Zero, shifted == 0);
        cpu.set_flag(StatusFlag::Negative, false);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::lsr::LRS;
    use crate::cpu::AddressingMode::{Accumulator, ZeroPage};

    #[test]
    fn shift_with_no_flags_acc() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0b00001010;

        // When
        LRS::new(Accumulator).execute(&mut cpu);

        // Then
        assert_eq!(0b00000101, cpu.accumulator);
        assert_eq!(0x00, cpu.processor_status);
    }

    #[test]
    fn shift_with_zero_and_carry_set() {
        // Given
        let mut cpu = CPU::new();
        cpu.write(&ZeroPage(0x88), 0x01);

        // When
        LRS::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.read(&ZeroPage(0x88)));
        assert_eq!(0x03, cpu.processor_status);   // Carry and Zero are set
    }

    #[test]
    fn negative_flag_always_cleared() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0xFF;
        cpu.processor_status = 0x80;

        // When
        LRS::new(Accumulator).execute(&mut cpu);

        // Then
        assert_eq!(0x7F, cpu.accumulator);
        assert_eq!(0x01, cpu.processor_status);   // Carry is set and negative is cleared
    }
}
