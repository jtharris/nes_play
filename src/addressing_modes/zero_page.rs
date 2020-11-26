use crate::cpu::{CPU, AddressingMode};

pub struct ZeroPage {
    address: u8
}

impl ZeroPage {
    pub fn new(address: u8) -> Self {
        ZeroPage{ address }
    }
}

impl AddressingMode for ZeroPage {
    fn read(&self, cpu: &CPU) -> u8 {
        cpu.read_mem8(self.address as u16)
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.write_mem8(self.address as u16, value);
    }
}

#[cfg(test)]
mod test {
    use super::ZeroPage;
    use crate::cpu::{CPU, AddressingMode};

    #[test]
    fn read_correct_value() {
        // Given
        let addressing_mode = ZeroPage::new(0xA8);
        let mut cpu = CPU::new();
        cpu.write_mem8(0xA8, 0x0C);

        // Then
        assert_eq!(0x0C, addressing_mode.read(&cpu));
    }

    #[test]
    fn write_correct_value() {
        // Given
        let addressing_mode = ZeroPage::new(0xA8);
        let mut cpu = CPU::new();

        // When
        addressing_mode.write(&mut cpu, 0xF1);

        // Then
        assert_eq!(0xF1, cpu.read_mem8(0xA8));
    }
}