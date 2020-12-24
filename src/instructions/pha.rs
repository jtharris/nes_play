use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#PHA
struct PHA {}

impl Instruction for PHA {
    fn execute(&self, cpu: &mut CPU) {
        cpu.push_stack(cpu.accumulator);
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::pha::PHA;

    #[test]
    fn acc_is_pushed() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0x8C;

        // When
        PHA{}.execute(&mut cpu);

        // Then
        assert_eq!(0x8C, cpu.pop_stack());
    }
}