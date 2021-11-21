use crate::cpu::{CPU, Instruction};

pub(super) enum JumpAddressMode {
    Absolute(u16),
    Indirect(u16)
}

// http://www.obelisk.me.uk/6502/reference.html#JMP
pub(super) struct JMP {
    mode: JumpAddressMode
}

impl JMP {
    pub fn new(mode: JumpAddressMode) -> Self {
        JMP{ mode }
    }
}

impl Instruction for JMP {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.program_counter = match self.mode {
            JumpAddressMode::Absolute(target) => target,
            JumpAddressMode::Indirect(address) => cpu.read_mem16(address)
        };

        match self.mode {
            JumpAddressMode::Absolute(_) => 3,
            JumpAddressMode::Indirect(_) => 5,
        }
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
        cpu.write_mem16(0x0183, 0x118C);

        // When
        JMP::new(Indirect(0x0183)).execute(&mut cpu);

        // Then
        assert_eq!(0x118C, cpu.program_counter);
    }
}
