use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, AddressingMode, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#ROL
pub(super) struct ROL {
    mode: AddressingMode
}

impl ROL {
    pub fn new(mode: AddressingMode) -> Self {
        ROL{ mode }
    }
}

impl Display for ROL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ROL {}", self.mode)
    }
}

impl Instruction for ROL {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let value = cpu.read(&self.mode);
        let new_carry = value > 0x7F;
        let mut new_value = value << 1;

        if cpu.get_flag(StatusFlag::Carry) {
            new_value |= 0x01;
        }

        cpu.write(&self.mode, new_value);
        cpu.set_flag(StatusFlag::Carry, new_carry);
        cpu.set_flag(StatusFlag::Zero, new_value == 0);
        cpu.set_flag(StatusFlag::Negative, new_value > 0x7F);

        cpu.memory_cycles(&self.mode)
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::ROL;
    use crate::cpu::AddressingMode::{Accumulator, ZeroPage};
    use crate::cpu::StatusFlag::Carry;

    #[test]
    fn rotate_acc_carry_0_to_1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xC5;

        // When
        ROL::new(Accumulator).execute(&mut cpu);

        // Then
        assert_eq!(0x8A, cpu.accumulator);
        assert_eq!(0x81, cpu.processor_status);  // carry is set because high bit was 1, also negative
    }

    #[test]
    fn rotate_acc_carry_1_to_0() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x10;
        cpu.set_flag(Carry, true);

        // When
        ROL::new(Accumulator).execute(&mut cpu);

        // Then
        assert_eq!(0x21, cpu.accumulator);
        assert_eq!(0x00, cpu.processor_status);  // carry cleared because high bit was 0
    }

    #[test]
    fn rotate_mem_with_zero() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xC5;
        cpu.write(&ZeroPage(0x8C), 0x80);

        // When
        ROL::new(ZeroPage(0x8C)).execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.read(&ZeroPage(0x8C)));
        assert_eq!(0x03, cpu.processor_status);  // carry is set because high bit was 1, also zero
    }

    #[test]
    fn string_representation() {
        let rol = ROL::new(Accumulator);

        assert_eq!("ROL A", rol.to_string())
    }
}