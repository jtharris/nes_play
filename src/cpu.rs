// http://wiki.nesdev.com/w/index.php/CPU_registers
pub struct CPU {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub index_register_x: u8,
    pub index_register_y: u8,
    pub processor_status: u8,  // http://wiki.nesdev.com/w/index.php/Status_flags
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: 0,
            memory: [0; 0xFFFF]
        }
    }

    pub fn read_mem8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_mem8(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn read_mem16(&self, addr: u16) -> u16 {
        let bytes = [self.read_mem8(addr), self.read_mem8(addr+1)];
        u16::from_le_bytes(bytes)
    }

    pub fn write_mem16(&mut self, addr: u16, val: u16) {
        let bytes: [u8; 2] = val.to_le_bytes();
        self.write_mem8(addr, bytes[0]);
        self.write_mem8(addr+1, bytes[1]);
    }
}

// Instructions are implemented as a visitor pattern, each being executable on
// a given CPU reference
// For instruction reference, see:  http://www.obelisk.me.uk/6502/reference.html
pub trait Instruction {
    fn execute(&self, cpu: &mut CPU);
}


// AddressingMode is a strategy for retrieving a value from memory
// See:  http://www.obelisk.me.uk/6502/addressing.html
// See:  https://skilldrick.github.io/easy6502/#addressing
// See:  http://wiki.nesdev.com/w/index.php/CPU_addressing_modes
pub trait AddressingMode {
    fn read(&self, cpu: &CPU) -> u8;
    fn write(&self, cpu: &CPU, value: u8);
}


#[cfg(test)]
mod test {
    use super::CPU;

    #[test]
    fn read_write_8bit_memory() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0xFF00, 0x1A);

        // Then
        assert_eq!(0x1A, cpu.read_mem8(0xFF00));
    }

    #[test]
    fn read_write_16bit_memory() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem16(0x0800, 0xFF9B);

        // Then
        assert_eq!(0xFF9B, cpu.read_mem16(0x0800));
    }

    #[test]
    fn read_16bit_memory_little_endian() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x08A0, 0x10);
        cpu.write_mem8(0x08A1, 0x28);

        // Then
        assert_eq!(0x2810, cpu.read_mem16(0x08A0));
    }
}
