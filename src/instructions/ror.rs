use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, AddressingMode, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#ROR
pub(super) struct ROR {
    mode: AddressingMode
}

impl ROR {
    pub fn new(mode: AddressingMode) -> Self {
        ROR{ mode }
    }
}

impl Display for ROR {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Instruction for ROR {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let value = cpu.read(&self.mode);
        let new_carry = value & 0x01 == 0x01;
        let mut new_value = value >> 1;

        if cpu.get_flag(StatusFlag::Carry) {
            new_value |= 0x80;
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
    use super::ROR;
    use crate::cpu::AddressingMode::{Accumulator, ZeroPage};
    use crate::cpu::StatusFlag::Carry;

    #[test]
    fn rotate_acc_carry_0_to_1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xC5;

        // When
        ROR::new(Accumulator).execute(&mut cpu);

        // Then
        assert_eq!(0x62, cpu.accumulator);
        assert_eq!(0x01, cpu.processor_status);  // carry is set because high bit was 1
    }

    #[test]
    fn rotate_acc_carry_1_to_0() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x10;
        cpu.set_flag(Carry, true);

        // When
        ROR::new(Accumulator).execute(&mut cpu);

        // Then
        assert_eq!(0x88, cpu.accumulator);
        assert_eq!(0x80, cpu.processor_status);  // carry cleared because low bit was 0, also negative
    }

    #[test]
    fn rotate_mem_with_zero() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xC5;
        cpu.write(&ZeroPage(0x8C), 0x01);

        // When
        ROR::new(ZeroPage(0x8C)).execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.read(&ZeroPage(0x8C)));
        assert_eq!(0x03, cpu.processor_status);  // carry is set because low bit was 1, also zero
    }
}