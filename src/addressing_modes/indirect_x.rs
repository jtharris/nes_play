use crate::cpu::{CPU, AddressingMode};

pub struct IndirectX {
    base_address: u8
}

impl IndirectX {
    pub fn new(base_address: u8) -> Self{
       IndirectX{ base_address }
    }
}

impl AddressingMode for IndirectX {
    fn read(&self, cpu: &CPU) -> u8 {
        let address1 = self.base_address.wrapping_add(cpu.index_register_x) as u16;
        let address = cpu.read_mem16(address1);

        cpu.read_mem8(address)
    }

    fn write(&self, cpu: &mut CPU, value: u8) {
        let address1 = self.base_address.wrapping_add(cpu.index_register_x) as u16;
        let address = cpu.read_mem16(address1);

        cpu.write_mem8(address, value);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode};
    use super::IndirectX;

    #[test]
    fn basic_read() {
        // Example from Indexed indirect here:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x01;
        cpu.write_mem8(0x01, 0x05);
        cpu.write_mem8(0x02, 0x07);
        cpu.write_mem8(0x0705, 0x0A);

        let addressing_mode = IndirectX::new(0x00);

        // Then
        assert_eq!(0x0A, addressing_mode.read(&cpu));
    }

    #[test]
    fn basic_write() {
        // Example derived from:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x01;
        cpu.write_mem8(0x01, 0x05);
        cpu.write_mem8(0x02, 0x07);

        let addressing_mode = IndirectX::new(0x00);

        // When
        addressing_mode.write(&mut cpu, 0x0A);

        // Then
        assert_eq!(0x0A, cpu.read_mem8(0x0705));
    }

}