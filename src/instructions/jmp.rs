use crate::cpu::{CPU, Instruction};

pub enum JumpAddressMode {
    Absolute(u16),
    Indirect(u16)
}

// http://www.obelisk.me.uk/6502/reference.html#JMP
pub struct JMP {
    mode: JumpAddressMode
}

impl JMP {
    pub fn new(mode: JumpAddressMode) -> Self {
        JMP{ mode }
    }
}

impl Instruction for JMP {
    fn execute(&self, cpu: &mut CPU) {
        cpu.program_counter = match self.mode {
            JumpAddressMode::Absolute(target) => target,
            JumpAddressMode::Indirect(address) => cpu.read_mem16(address)
        };
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::{JMP, JumpAddressMode::*};

    #[test]
    fn absolute_jump() {
        // Given
        let mut cpu = CPU::new();

        // When
        JMP::new(Absolute(0xA281)).execute(&mut cpu);

        // Then
        assert_eq!(0xA281, cpu.program_counter);
    }

    #[test]
    fn indirect_jump() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem16(0xFF83, 0x118C);

        // When
        JMP::new(Indirect(0xFF83)).execute(&mut cpu);

        // Then
        assert_eq!(0x118C, cpu.program_counter);
    }
}
