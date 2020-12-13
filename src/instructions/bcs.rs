use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#BCS
struct BCS {
    relative: i8
}

impl BCS {
    pub fn new(relative: i8) -> Self {
        BCS{ relative }
    }
}

impl Instruction for BCS {
    fn execute(&self, cpu: &mut CPU) {
        if cpu.get_flag(StatusFlag::Carry) {
            cpu.program_counter = ((cpu.program_counter as i16) + (self.relative as i16)) as u16;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BCS;

    #[test]
    fn no_effect() {
        // Given
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x02;    // Carry is clear

        // When
        let bcc = BCS::new(0xA);
        bcc.execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x0844);   // Nothing changed
        assert_eq!(cpu.processor_status, 0x02);
    }

    #[test]
    fn jump_forward() {
        // Given
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0844;
        cpu.processor_status = 0x01;

        // When
        let bcc = BCS::new(0xA);
        bcc.execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0x084E);
        assert_eq!(cpu.processor_status, 0x01);
    }

    #[test]
    fn jump_back() {
        // Given
        let mut cpu = CPU::new();
        cpu.program_counter = 0xF844;
        cpu.processor_status = 0x01;

        // When
        let bcc = BCS::new(-0xF);
        bcc.execute(&mut cpu);

        // Then
        assert_eq!(cpu.program_counter, 0xF835);
        assert_eq!(cpu.processor_status, 0x01);
    }
}
