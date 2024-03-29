use std::fmt::{Display, format, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BCS
pub(super) struct BCS {
    relative: i8
}

impl BCS {
    pub fn new(relative: i8) -> Self {
        BCS{ relative }
    }
}

impl Display for BCS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCS ${:02X}", self.relative)
    }
}

impl Instruction for BCS {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if cpu.get_flag(StatusFlag::Carry) {
            let new_pc = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
            3 + cpu.set_pc(new_pc)
        } else {
            2
        }
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0xB0, self.relative as u8]
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        let new_pc = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
        format!("BCS ${:04X}", new_pc)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BCS;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x02;    // Carry is clear

        // When
        BCS::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x0844);   // Nothing changed
        assert_eq!(cpu.processor_status, 0x02);
    }

    #[test]
    fn jump_forward() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x01;

        // When
        BCS::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x01);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x01;

        // When
        BCS::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x01);
    }

    #[test]
    fn string_representation_positive() {
        // Given
        let bcs = BCS::new(0x2A);

        // Then
        assert_eq!("BCS $2A", bcs.to_string())
    }

    #[test]
    fn string_representation_negative() {
        // Given
        let bcs = BCS::new(-0x13);

        // Then
        assert_eq!("BCS $ED", bcs.to_string())
    }

    #[test]
    fn bytes_representation() {
        // Given
        let bsc = BCS::new(-0x17);

        // Then
        assert_eq!(vec![0xB0, 0xE9], bsc.bytes());
    }
}
