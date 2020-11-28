use crate::cpu::{CPU, AddressingMode};

pub struct IndirectY {
    base_address: u8
}

impl IndirectY {
    pub fn new(base_address: u8) -> Self{
       IndirectY{ base_address }
    }
}

impl AddressingMode for IndirectY {
    fn read(&self, cpu: &CPU) -> u8 {
        let address1 = cpu.read_mem16(self.base_address as u16);
        let address = address1.wrapping_add(cpu.index_register_y as u16);

        cpu.read_mem8(address)
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        let address1 = cpu.read_mem16(self.base_address as u16);
        let address = address1.wrapping_add(cpu.index_register_y as u16);

        cpu.write_mem8(address, value);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode};
    use super::IndirectY;

    #[test]
    fn basic_read() {
        // Example from Indexed indirect here:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::new();

        cpu.index_register_y = 0x01;
        cpu.write_mem8(0x01, 0x03);
        cpu.write_mem8(0x02, 0x07);
        cpu.write_mem8(0x0704, 0x0A);

        let addressing_mode = IndirectY::new(0x01);

        // Then
        assert_eq!(0x0A, addressing_mode.read(&cpu));
    }

    #[test]
    fn basic_write() {
        // Adapted from the basic_read
        // Given
        let mut cpu = CPU::new();

        cpu.index_register_y = 0x01;
        cpu.write_mem8(0x01, 0x03);
        cpu.write_mem8(0x02, 0x07);

        let addressing_mode = IndirectY::new(0x01);

        // When
        addressing_mode.write(&mut cpu, 0x0A);

        // Then
        assert_eq!(0x0A, cpu.read_mem8(0x0704));
    }
}