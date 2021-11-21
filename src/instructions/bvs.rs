use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BVS
pub(super) struct BVS {
    relative: i8
}

impl BVS {
    pub fn new(relative: i8) -> Self {
        BVS{ relative }
    }
}

impl Instruction for BVS {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if cpu.get_flag(StatusFlag::Overflow) {
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
    use super::BVS;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x01;

        // When
        BVS::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x0844);   // Nothing changed
        assert_eq!(cpu.processor_status, 0x01);
    }

    #[test]
    fn jump_forward() {
        // Given
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x40;

        // When
        BVS::new(0xA).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x40);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::new();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0xC2;

        // When
        BVS::new(-0xF).execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0xC2);
    }
}
