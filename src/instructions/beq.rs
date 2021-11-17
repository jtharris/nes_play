use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BEQ
struct BEQ {
    relative: i8
}

impl BEQ {
    pub fn new(relative: i8) -> Self {
        BEQ{ relative }
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
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BEQ;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::new();
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
        let mut cpu = CPU::new();
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
        let mut cpu = CPU::new();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x02;

        // When
        BEQ::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x02);
    }
}
