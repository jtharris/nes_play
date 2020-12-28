use crate::cpu::{AddressingMode, Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#STX
struct STX {
    mode: AddressingMode
}

impl STX {
    pub fn new(mode: AddressingMode) -> Self {
        STX{ mode }
    }
}

impl Instruction for STX {
    fn execute(&self, cpu: &mut CPU) {
        cpu.write(&self.mode, cpu.index_register_x);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::ZeroPage;
    use super::STX;

    #[test]
    fn x_is_stored() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0xA7;

        // When
        STX::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0xA7, cpu.read(&ZeroPage(0x88)))
    }
}
