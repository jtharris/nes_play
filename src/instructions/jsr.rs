use crate::cpu::{CPU, Instruction};

// http://www.obelisk.me.uk/6502/reference.html#JSR
pub struct JSR {
    target: u16
}

impl JSR {
    pub fn new(target: u16) -> Self {
        JSR{ target }
    }
}

impl Instruction for JSR {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        // I *think* using big endian is due to the stack decrementing instead of incrementing...?
        let return_address = cpu.program_counter - 1;
        let bytes: [u8; 2] = return_address.to_be_bytes();
        cpu.push_stack(bytes[0]);
        cpu.push_stack(bytes[1]);
        cpu.program_counter = self.target;

        6
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction, AddressingMode::Absolute};
    use super::JSR;

    #[test]
    fn jump_to_sub() {
        // Given
        let mut cpu = CPU::new();
        cpu.program_counter = 0xD8DC;

        // When
        JSR::new(0x02F0).execute(&mut cpu);

        // Then
        assert_eq!(0x02F0, cpu.program_counter);
        assert_eq!(0xD8, cpu.read(&Absolute(0x01FD)));
        assert_eq!(0xDB, cpu.read(&Absolute(0x01FC)));  // Once loaded, PC will inc
    }
}
