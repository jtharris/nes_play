use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BEQ
pub(super) struct BEQ {
    relative: i8
}

impl BEQ {
    pub fn new(relative: i8) -> Self {
        BEQ{ relative }
    }
}

impl Display for BEQ {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BEQ ${:02X}", self.relative)
    }
}

impl Instruction for BEQ {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if cpu.get_flag(StatusFlag::Zero) {
            let new_pc = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
            3 + cpu.set_pc(new_pc)
        } else {
            2
        }
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0xF0, self.relative as u8]
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        let new_pc = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
        format!("BEQ ${:04X}", new_pc)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BEQ;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x01;    // Zero is clear

        // When
        BEQ::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x0844);   // Nothing changed
        assert_eq!(cpu.processor_status, 0x01);
    }

    #[test]
    fn jump_forward() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x02;

        // When
        BEQ::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x02);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x02;

        // When
        BEQ::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x02);
    }

    #[test]
    fn string_representation_positive() {
        // Given
        let beq = BEQ::new(0x30);

        // Then
        assert_eq!("BEQ $30", beq.to_string())
    }

    #[test]
    fn string_representation_negative() {
        // Given
        let beq = BEQ::new(-0x01);

        // Then
        assert_eq!("BEQ $FF", beq.to_string())
    }

    #[test]
    fn bytes_representation() {
        // Given
        let beq = BEQ::new(0x77);

        // Then
        assert_eq!(vec![0xF0, 0x77], beq.bytes());
    }
}
