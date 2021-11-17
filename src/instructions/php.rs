use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#PHP
pub(super) struct PHP {}

impl Instruction for PHP {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        // See http://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag
        // for description of B-flag behavior
        cpu.push_stack(cpu.processor_status | 0b0001_1000);

        3
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::php::PHP;

    #[test]
    fn s_is_pushed_with_b_flag() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0b1000_0001;

        // When
        PHP{}.execute(&mut cpu);

        // Then
        assert_eq!(0b1000_0001, cpu.processor_status);  // PC remains the same
        assert_eq!(0b1001_1001, cpu.pop_stack());
    }

    #[test]
    fn s_is_pushed_no_change() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0b1011_1101;

        // When
        PHP{}.execute(&mut cpu);

        // Then
        assert_eq!(0b1011_1101, cpu.processor_status);  // PC remains the same
        assert_eq!(0b1011_1101, cpu.pop_stack());
    }
}