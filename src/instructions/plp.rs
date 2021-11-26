use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#PLP
pub(super) struct PLP {}

impl Display for PLP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PLP")
    }
}

impl Instruction for PLP {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.processor_status = cpu.pop_stack();

        4
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::plp::PLP;

    #[test]
    fn s_is_pulled() {
        // Given
        let mut cpu = CPU::empty();
        cpu.push_stack(0xAF);

        // When
        PLP{}.execute(&mut cpu);

        // Then
        assert_eq!(0xAF, cpu.processor_status);
    }
}