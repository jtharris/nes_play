use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BPL
pub(super) struct BPL {
    relative: i8
}

impl BPL {
    pub fn new(relative: i8) -> Self {
        BPL{ relative }
    }
}

impl Display for BPL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BPL ${:02X}", self.relative)
    }
}

impl Instruction for BPL {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if !cpu.get_flag(StatusFlag::Negative) {
            let new_pc = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
            3 + cpu.set_pc(new_pc)
        } else {
            2
        }
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0x10, self.relative as u8]
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BPL;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x80;    // Negative flag is set

        // When
        BPL::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x0844);   // Nothing changed
        assert_eq!(cpu.processor_status, 0x80);
    }

    #[test]
    fn jump_forward() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x03;

        // When
        BPL::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x03);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x00;

        // When
        BPL::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x00);
    }

    #[test]
    fn string_representation_positive() {
        // Given
        let bpl = BPL::new(0x1F);

        // Then
        assert_eq!("BPL $1F", bpl.to_string())
    }

    #[test]
    fn string_representation_negative() {
        // Given
        let bpl = BPL::new(-0x1A);

        // Then
        assert_eq!("BPL $E6", bpl.to_string())
    }

    #[test]
    fn bytes_representation() {
        // Given
        let bpl = BPL::new(0x02);

        // Then
        assert_eq!(vec![0x10, 0x02], bpl.bytes());
    }
}
