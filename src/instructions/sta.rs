use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#STA
pub(super) struct STA {
    mode: AddressingMode
}

impl STA {
    pub fn new(mode: AddressingMode) -> Self {
        STA{ mode }
    }
}

impl Display for STA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "STA {}", self.mode)
    }
}

impl Instruction for STA {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.write(&self.mode, cpu.accumulator);

        match self.mode {
            AddressingMode::ZeroPage(_) => 3,
            AddressingMode::ZeroPageX(_) => 4,
            AddressingMode::Absolute(_) => 4,
            AddressingMode::AbsoluteX(_) => 5,
            AddressingMode::AbsoluteY(_) => 5,
            AddressingMode::IndirectX(_) => 6,
            AddressingMode::IndirectY(_) => 6,
            _ => panic!("Invalid addressing mode for STA")
        }
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0x85, addr],
            AddressingMode::ZeroPageX(addr) => vec![0x95, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0x8D, addr),
            AddressingMode::AbsoluteX(addr) => self.bytes_for_opcode(0x9D, addr),
            AddressingMode::AbsoluteY(addr) => self.bytes_for_opcode(0x99, addr),
            AddressingMode::IndirectX(addr) => vec![0x81, addr],
            AddressingMode::IndirectY(addr) => vec![0x91, addr],
            _ => panic!("Addressing mode not allowed for STA")
        }
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        format!("STA {}", self.mode.debug_string(&cpu))
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{Absolute, IndirectY, ZeroPage};
    use super::STA;

    #[test]
    fn acc_is_stored() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xA7;

        // When
        STA::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0xA7, cpu.read(&ZeroPage(0x88)))
    }

    #[test]
    fn absolute_address_mode() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x12;

        // When
        STA::new(Absolute(0x0245)).execute(&mut cpu);

        // Then
        assert_eq!(0x12, cpu.read(&Absolute(0x0245)));
    }

    #[test]
    fn string_representation() {
        let sta = STA::new(IndirectY(0x0C));

        assert_eq!("STA ($0C),Y", sta.to_string())
    }

    #[test]
    fn bytes_representation() {
        let sta = STA::new(Absolute(0x0245));

        assert_eq!(vec![0x8D, 0x45, 0x02], sta.bytes());
    }
}
