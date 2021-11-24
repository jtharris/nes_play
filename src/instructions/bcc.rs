use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BCC
pub(super) struct BCC {
    relative: i8
}

impl BCC {
    pub fn new(relative: i8) -> Self {
        BCC{ relative }
    }
}

impl Display for BCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCC ${:02X}", self.relative)
    }
}

impl Instruction for BCC {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if !cpu.get_flag(StatusFlag::Carry) {
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
    use super::BCC;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x03;

        // When
        BCC::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x0844);   // Nothing changed
        assert_eq!(cpu.processor_status, 0x03);
    }

    #[test]
    fn jump_forward() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x00;

        // When
        BCC::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x00);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x00;

        // When
        BCC::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x00);
    }

    #[test]
    fn string_representation_positive() {
        // Given
        let bcc = BCC::new(0x0A);

        // Then
        assert_eq!("BCC $0A", bcc.to_string())
    }

    #[test]
    fn string_representation_negative() {
        // Given
        let bcc = BCC::new(-0x03);

        // Then
        assert_eq!("BCC $FD", bcc.to_string())
    }
}
