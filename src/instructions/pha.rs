use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#PHA
pub(super) struct PHA {}

impl Display for PHA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Instruction for PHA {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.push_stack(cpu.accumulator);

        3
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::pha::PHA;

    #[test]
    fn acc_is_pushed() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x8C;

        // When
        PHA{}.execute(&mut cpu);

        // Then
        assert_eq!(0x8C, cpu.pop_stack());
    }
}