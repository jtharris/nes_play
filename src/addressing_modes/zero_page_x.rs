use crate::cpu::{CPU, AddressingMode};

pub struct ZeroPageX {
    base_address: u8
}

impl ZeroPageX {
    pub fn new(base_address: u8) -> Self{
       ZeroPageX{ base_address }
    }
}

impl AddressingMode for ZeroPageX {
    fn read(&self, cpu: &CPU) -> u8 {
        let address = self.base_address.wrapping_add(cpu.index_register_x);
        cpu.read_mem8(address as u16)
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        let address = self.base_address.wrapping_add(cpu.index_register_x);
        cpu.write_mem8(address as u16, value);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode};
    use super::ZeroPageX;

    #[test]
    fn basic_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x8D, 0x09);
        cpu.index_register_x = 0x0A;

        let addressing_mode = ZeroPageX::new(0x83);

        // Then
        assert_eq!(0x09, addressing_mode.read(&cpu));
    }

    #[test]
    fn wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x04, 0x29);
        cpu.index_register_x = 0x05;

        let addressing_mode = ZeroPageX::new(0xFF);

        // Then
        assert_eq!(0x29, addressing_mode.read(&cpu));
    }

    #[test]
    fn basic_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x0A;

        let addressing_mode = ZeroPageX::new(0x83);
        addressing_mode.write(&mut cpu, 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x8D));
    }

    #[test]
    fn wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x07;

        let addressing_mode = ZeroPageX::new(0xFF);
        addressing_mode.write(&mut cpu, 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x06));
    }
}