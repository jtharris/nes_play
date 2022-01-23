use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#DEC
pub struct DEC {
    mode: AddressingMode
}

impl DEC {
    pub fn new(mode: AddressingMode) -> Self {
        DEC{ mode }
    }
}

impl Display for DEC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DEC {}", self.mode)
    }
}

impl Instruction for DEC {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let (val, _) = cpu.read(&self.mode).overflowing_sub(1);
        cpu.write(&self.mode, val);

        cpu.set_flag(StatusFlag::Zero,  val == 0);
        cpu.set_flag(StatusFlag::Negative, val > 0x7F);

        cpu.memory_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0xC6, addr],
            AddressingMode::ZeroPageX(addr) => vec![0xD6, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0xCE, addr),
            AddressingMode::AbsoluteX(addr) => self.bytes_for_opcode(0xDE, addr),
            _ => panic!("Addressing mode not allowed for DEC")
        }
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        format!("DEC {}", self.mode.debug_string(&cpu))
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{Absolute, ZeroPage, ZeroPageX};
    use crate::instructions::dec::DEC;

    #[test]
    fn basic_decrement() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xAA);
        cpu.write(&mode, 0x0C);

        // When
        DEC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x0B, cpu.read(&ZeroPage(0xAA)));
        assert_eq!(0, cpu.processor_status);    // Make sure no bits were set
    }

    #[test]
    fn zero_result() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0x09);
        cpu.write(&mode, 0x01);

        // When
        DEC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0, cpu.read(&ZeroPage(0x09)));
        assert_eq!(0x02, cpu.processor_status);    // Zero flag is set
    }

    #[test]
    fn negative_result() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0x04);
        cpu.write(&mode, 0xFD);

        // When
        DEC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0xFC, cpu.read(&ZeroPage(0x04)));
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }

    #[test]
    fn negative_wrap() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0x04);
        cpu.write(&mode, 0x00);

        // When
        DEC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0xFF, cpu.read(&ZeroPage(0x04)));
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }

    #[test]
    fn string_representation() {
        let dec = DEC::new(ZeroPageX(0x8D));

        assert_eq!("DEC $8D,X", dec.to_string())
    }

    #[test]
    fn bytes_representation() {
        let dec = DEC::new(Absolute(0xFE01));

        assert_eq!(vec![0xCE, 0x01, 0xFE], dec.bytes());
    }
}
