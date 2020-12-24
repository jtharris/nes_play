use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#PLP
struct PLP {}

impl Instruction for PLP {
    fn execute(&self, cpu: &mut CPU) {
        cpu.processor_status = cpu.pop_stack();
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::plp::PLP;

    #[test]
    fn s_is_pulled() {
        // Given
        let mut cpu = CPU::new();
        cpu.push_stack(0xAF);

        // When
        PLP{}.execute(&mut cpu);

        // Then
        assert_eq!(0xAF, cpu.processor_status);
    }
}