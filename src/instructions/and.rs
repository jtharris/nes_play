use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#AND
struct AND {
    mode: AddressingMode
}

impl AND {
    pub fn new(mode: AddressingMode) -> Self {
        AND{ mode }
    }
}

impl Instruction for AND {
    fn execute(&self, cpu: &mut CPU) {
        let memory_value= cpu.read(&self.mode);
        cpu.accumulator &= memory_value;

        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate};
    use crate::instructions::and::AND;

    #[test]
    fn basic_and() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xD3);

        cpu.accumulator = 0b01101110;
        cpu.write(&mode, 0b00110111);

        // When
        let and = AND::new(mode);
        and.execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0b00100110);
        assert_eq!(cpu.processor_status, 0);
    }

    #[test]
    fn zero_flag_and() {
        // Given
        let mut cpu = CPU::new();
        let mode = Immediate(0);

        cpu.accumulator = 0b01101110;

        // When
        let and = AND::new(mode);
        and.execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.processor_status, 0b00000010);  // Zero flag is set
    }

    #[test]
    fn negative_flag_and() {
        // Given
        let mut cpu = CPU::new();
        let mode = Immediate(0xF0);

        cpu.accumulator = 0b11011110;

        // When
        let and = AND::new(mode);
        and.execute(&mut cpu);

        // Then
        assert_eq!(cpu.accumulator, 0b11010000);
        assert_eq!(cpu.processor_status, 0b10000000);  // Negative flag is set
    }
}
