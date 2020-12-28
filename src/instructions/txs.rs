use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#TXS
struct TXS {}

impl Instruction for TXS {
    fn execute(&self, cpu: &mut CPU) {
        cpu.stack_pointer = cpu.index_register_x;
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::TXS;

    #[test]
    fn copies_to_stack() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x7E;

        // When
        TXS{}.execute(&mut cpu);

        // Then
        assert_eq!(0x7E, cpu.stack_pointer);
        assert_eq!(0x00, cpu.processor_status);
    }
}
