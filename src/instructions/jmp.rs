use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction};
use crate::cpu::AddressingMode::Absolute;

pub(super) enum JumpAddressMode {
    Absolute(u16),
    Indirect(u16)
}

impl Display for JumpAddressMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpAddressMode::Absolute(addr) => write!(f, "${:04X}", addr),
            JumpAddressMode::Indirect(addr) => write!(f, "(${:04X})", addr),
        }
    }
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

impl Display for JMP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JMP {}", self.mode)
    }
}

impl Instruction for JMP {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.program_counter = match self.mode {
            JumpAddressMode::Absolute(target) => target,
            JumpAddressMode::Indirect(address) => {
                // See here for explanation:
                // http://www.6502.org/tutorials/6502opcodes.html#JMP
                let address_parts = address.to_be_bytes();
                let high_byte_address = u16::from_be_bytes([
                    address_parts[0],
                    address_parts[1].wrapping_add(1)
                ]);

                let bytes = [
                    cpu.read(&Absolute(address)),
                    cpu.read(&Absolute(high_byte_address))
                ];
                u16::from_le_bytes(bytes)
            }
        };

        match self.mode {
            JumpAddressMode::Absolute(_) => 3,
            JumpAddressMode::Indirect(_) => 5,
        }
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{AddressingMode, CPU, Instruction};
    use super::{JMP, JumpAddressMode::*};

    #[test]
    fn absolute_jump() {
        // Given
        let mut cpu = CPU::empty();

        // When
        JMP::new(Absolute(0xA281)).execute(&mut cpu);

        // Then
        assert_eq!(0xA281, cpu.program_counter);
    }

    #[test]
    fn indirect_jump() {
        // Given
        let mut cpu = CPU::empty();
        cpu.write_mem16(0x0183, 0x118C);

        // When
        JMP::new(Indirect(0x0183)).execute(&mut cpu);

        // Then
        assert_eq!(0x118C, cpu.program_counter);
    }

    #[test]
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();

        cpu.write(&AddressingMode::Absolute(0x02FF), 0x00);
        cpu.write(&AddressingMode::Absolute(0x0300), 0xA9);
        cpu.write(&AddressingMode::Absolute(0x0200), 0x03);
        cpu.write(&AddressingMode::Absolute(0x0301), 0xAA);
        cpu.write(&AddressingMode::Absolute(0x0302), 0x60);

        // When
        JMP::new(Indirect(0x02FF)).execute(&mut cpu);

        // Then
        assert_eq!(0x0300, cpu.program_counter);
    }

    #[test]
    fn string_representation_absolute() {
        let jmp = JMP::new(Absolute(0x5597));

        assert_eq!("JMP $5597", jmp.to_string())
    }

    #[test]
    fn string_representation_indirect() {
        let jmp = JMP::new(Indirect(0x5597));

        assert_eq!("JMP ($5597)", jmp.to_string())
    }
}
