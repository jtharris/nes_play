use crate::cpu::{CPU, AddressingMode};

pub struct AbsoluteX {
    base_address: u16
}

impl AbsoluteX {
    pub fn new(base_address: u16) -> Self{
       AbsoluteX{ base_address }
    }
}

impl AddressingMode for AbsoluteX {
    fn read(&self, cpu: &CPU) -> u8 {
        let address = self.base_address.wrapping_add(cpu.index_register_x as u16);
        cpu.read_mem8(address)
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        let address = self.base_address.wrapping_add(cpu.index_register_x as u16);
        cpu.write_mem8(address, value);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode};
    use super::AbsoluteX;

    #[test]
    fn basic_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x8D, 0x09);
        cpu.index_register_x = 0x0A;

        let addressing_mode = AbsoluteX::new(0x83);

        // Then
        assert_eq!(0x09, addressing_mode.read(&cpu));
    }

    #[test]
    fn no_wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x0104, 0x29);
        cpu.index_register_x = 0x05;

        let addressing_mode = AbsoluteX::new(0x00FF);

        // Then
        assert_eq!(0x29, addressing_mode.read(&cpu));
    }

    #[test]
    fn wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x0003, 0x29);
        cpu.index_register_x = 0x05;

        let addressing_mode = AbsoluteX::new(0xFFFE);

        // Then
        assert_eq!(0x29, addressing_mode.read(&cpu));
    }

    #[test]
    fn basic_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x0A;

        let addressing_mode = AbsoluteX::new(0x83);
        addressing_mode.write(&mut cpu, 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x8D));
    }

    #[test]
    fn no_wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x07;

        let addressing_mode = AbsoluteX::new(0x00FF);
        addressing_mode.write(&mut cpu, 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x0106));
    }

    #[test]
    fn wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x07;

        let addressing_mode = AbsoluteX::new(0xFFFD);
        addressing_mode.write(&mut cpu, 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x0004));
    }
}