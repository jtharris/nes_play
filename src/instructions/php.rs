use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#PHP
struct PHP {}

impl Instruction for PHP {
    fn execute(&self, cpu: &mut CPU) {
        cpu.push_stack(cpu.processor_status);
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::php::PHP;

    #[test]
    fn s_is_pushed() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0x81;

        // When
        PHP{}.execute(&mut cpu);

        // Then
        assert_eq!(0x81, cpu.pop_stack());
    }
}