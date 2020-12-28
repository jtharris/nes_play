use crate::cpu::{AddressingMode, Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#STY
struct STY {
    mode: AddressingMode
}

impl STY {
    pub fn new(mode: AddressingMode) -> Self {
        STY{ mode }
    }
}

impl Instruction for STY {
    fn execute(&self, cpu: &mut CPU) {
        cpu.write(&self.mode, cpu.index_register_y);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::ZeroPage;
    use super::STY;

    #[test]
    fn y_is_stored() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_y = 0xA7;

        // When
        STY::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0xA7, cpu.read(&ZeroPage(0x88)))
    }
}
