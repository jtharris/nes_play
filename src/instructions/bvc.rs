use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BVC
pub(super) struct BVC {
    relative: i8
}

impl BVC {
    pub fn new(relative: i8) -> Self {
        BVC{ relative }
    }
}

impl Display for BVC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BVC ${:02X}", self.relative)
    }
}

impl Instruction for BVC {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if !cpu.get_flag(StatusFlag::Overflow) {
            let new_pc = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
            3 + cpu.set_pc(new_pc)
        } else {
            2
        }
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BVC;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x40;

        // When
        BVC::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x0844);   // Nothing changed
        assert_eq!(cpu.processor_status, 0x40);
    }

    #[test]
    fn jump_forward() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x00;

        // When
        BVC::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x00);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x02;

        // When
        BVC::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x02);
    }

    #[test]
    fn string_representation_positive() {
        // Given
        let bvc = BVC::new(0x10);

        // Then
        assert_eq!("BVC $10", bvc.to_string())
    }

    #[test]
    fn string_representation_negative() {
        // Given
        let bvc = BVC::new(-0x0A);

        // Then
        assert_eq!("BVC $F6", bvc.to_string())
    }
}
