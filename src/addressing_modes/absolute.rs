use crate::cpu::{CPU, AddressingMode};

pub struct Absolute {
    address: u16
}

impl Absolute {
    pub fn new(address: u16) -> Self {
        Absolute{ address }
    }
}

impl AddressingMode for Absolute {
    fn read(&self, cpu: &CPU) -> u8 {
       cpu.read_mem8(self.address)
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.write_mem8(self.address, value);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode};
    use super::Absolute;

    #[test]
    fn basic_read() {
        // Given
        let addressing_mode = Absolute::new(0xF9EA);
        let mut cpu = CPU::new();

        cpu.write_mem8(0xF9EA, 0x8A);

        // Then
        assert_eq!(0x8A, addressing_mode.read(&cpu));
    }

    #[test]
    fn basic_write() {
        // Given
        let addressing_mode = Absolute::new(0xF9EA);
        let mut cpu = CPU::new();

        // When
        addressing_mode.write(&mut cpu, 0x07);

        // Then
        assert_eq!(0x07, cpu.read_mem16(0xF9EA));
    }
}