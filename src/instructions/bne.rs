use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BNE
pub(super) struct BNE {
    relative: i8
}

impl BNE {
    pub fn new(relative: i8) -> Self {
        BNE{ relative }
    }
}

impl Display for BNE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BNE ${:02X}", self.relative)
    }
}

impl Instruction for BNE {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if !cpu.get_flag(StatusFlag::Zero) {
            let new_pc = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
            3 + cpu.set_pc(new_pc)
        } else {
            2
        }
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0xD0, self.relative as u8]
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BNE;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x02;    // Zero is set

        // When
        BNE::new(0xA).execute(&mut cpu);

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
        BNE::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x01);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::empty();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x00;

        // When
        BNE::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x00);
    }

    #[test]
    fn string_representation_positive() {
        // Given
        let bne = BNE::new(0x1C);

        // Then
        assert_eq!("BNE $1C", bne.to_string())
    }

    #[test]
    fn string_representation_negative() {
        // Given
        let bne = BNE::new(-0x11);

        // Then
        assert_eq!("BNE $EF", bne.to_string())
    }

    #[test]
    fn bytes_representation() {
        // Given
        let bne = BNE::new(-0x10);

        // Then
        assert_eq!(vec![0xD0, 0xF0], bne.bytes());
    }
}
