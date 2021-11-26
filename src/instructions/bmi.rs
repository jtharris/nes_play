use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BMI
pub(super) struct BMI {
    relative: i8
}

impl BMI {
    pub fn new(relative: i8) -> Self {
        BMI{ relative }
    }
}

impl Display for BMI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BMI ${:02X}", self.relative)
    }
}

impl Instruction for BMI {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if cpu.get_flag(StatusFlag::Negative) {
            let new_pc = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
            3 + cpu.set_pc(new_pc)
        } else {
            2
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BMI;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x00;    // Negative flag is clear

        // When
        BMI::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x0844);   // Nothing changed
        assert_eq!(cpu.processor_status, 0x00);
    }

    #[test]
    fn jump_forward() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x81;

        // When
        BMI::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x81);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x80;

        // When
        BMI::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn string_representation_positive() {
        // Given
        let bmi = BMI::new(0x3A);

        // Then
        assert_eq!("BMI $3A", bmi.to_string())
    }

    #[test]
    fn string_representation_negative() {
        // Given
        let bmi = BMI::new(-0x0A);

        // Then
        assert_eq!("BMI $F6", bmi.to_string())
    }
}
