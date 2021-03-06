use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#ORA
struct ORA {
    mode: AddressingMode
}

impl ORA {
    pub fn new(mode: AddressingMode) -> Self {
        ORA{ mode }
    }
}

impl Instruction for ORA {
    fn execute(&self, cpu: &mut CPU) {
        cpu.accumulator |= cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate};
    use crate::instructions::ora::ORA;

    #[test]
    fn basic_or() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xD3);

        cpu.accumulator = 0b01101110;
        cpu.write(&mode, 0b00110111);

        // When
        ORA::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0b01111111);
        assert_eq!(cpu.processor_status, 0);
    }

    #[test]
    fn zero_flag_or() {
        // Given
        let mut cpu = CPU::new();
        let mode = Immediate(0x00);

        cpu.accumulator = 0x00;

        // When
        ORA::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.processor_status, 0b00000010);  // Zero flag is set
    }

    #[test]
    fn negative_flag_or() {
        // Given
        let mut cpu = CPU::new();
        let mode = Immediate(0x01);

        cpu.accumulator = 0b11011110;

        // When
        ORA::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0b11011111);
        assert_eq!(cpu.processor_status, 0b10000000);  // Negative flag is set
    }
}
