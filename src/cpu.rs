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

    // http://wiki.nesdev.com/w/index.php/Status_flags
    pub fn set_flag(&mut self, flag: StatusFlag, value: bool) {
        let bit_mask: u8 = match flag {
            StatusFlag::Carry =>            0b00000001,
            StatusFlag::Zero =>             0b00000010,
            StatusFlag::InterruptDisable => 0b00000100,
            StatusFlag::Decimal =>          0b00001000,
            StatusFlag::Overflow =>         0b01000000,
            StatusFlag::Negative =>         0b10000000
        };

        self.processor_status = match value {
            true => self.processor_status | bit_mask,
            false => self.processor_status & !bit_mask,
        };
    }

    fn read_mem8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write_mem8(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    fn read_mem16(&self, addr: u16) -> u16 {
        let bytes = [self.read_mem8(addr), self.read_mem8(addr+1)];
        u16::from_le_bytes(bytes)
    }

    fn write_mem16(&mut self, addr: u16, val: u16) {
        let bytes: [u8; 2] = val.to_le_bytes();
        self.write_mem8(addr, bytes[0]);
        self.write_mem8(addr+1, bytes[1]);
    }

    // See for reference:
    //  * www.obelisk.me.uk/6502/addressing.html
    //  * http://wiki.nesdev.com/w/index.php/CPU_addressing_modes
    //  * https://skilldrick.github.io/easy6502/#addressing
    fn mem_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            &AddressingMode::Absolute(address) => address,
            &AddressingMode::AbsoluteX(base) => {
                base.wrapping_add(self.index_register_x as u16)
            }
            &AddressingMode::AbsoluteY(base) => {
                base.wrapping_add(self.index_register_y as u16)
            }
            &AddressingMode::IndirectX(base) => {
                self.read_mem16(base.wrapping_add(self.index_register_x) as u16)
            }
            &AddressingMode::IndirectY(address) => {
                self.read_mem16(address as u16).wrapping_add(self.index_register_y as u16)
            }
            &AddressingMode::ZeroPage(address) => address as u16,
            &AddressingMode::ZeroPageX(base) => {
                base.wrapping_add(self.index_register_x) as u16
            }
            &AddressingMode::ZeroPageY(base) => {
                base.wrapping_add(self.index_register_y) as u16
            }
            &AddressingMode::Immediate(_) => panic!("Immediate Addressing Mode has no memory address"),
            &AddressingMode::Accumulator => panic!("Accumulator Mode has no memory address"),
        }
    }

    pub fn read(&self, mode: &AddressingMode) -> u8 {
        match mode {
            &AddressingMode::Accumulator => self.accumulator,
            &AddressingMode::Immediate(value) => value,
            am => self.read_mem8(self.mem_address(am))
        }
    }

    pub fn write(&mut self, mode: &AddressingMode, value: u8) {
        match mode {
            &AddressingMode::Accumulator => self.accumulator = value,
            &AddressingMode::Immediate(_) => {},
            am => self.write_mem8(self.mem_address(am), value)
        }
    }
}


// http://wiki.nesdev.com/w/index.php/Status_flags
pub enum StatusFlag {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    Overflow,
    Negative,
}


// AddressingMode is a strategy for retrieving a value from memory
// See:  http://www.obelisk.me.uk/6502/addressing.html
// See:  https://skilldrick.github.io/easy6502/#addressing
// See:  http://wiki.nesdev.com/w/index.php/CPU_addressing_modes
pub enum AddressingMode {
    Accumulator,
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Immediate(u8),
    IndirectX(u8),
    IndirectY(u8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
}


// Instructions are implemented as a visitor pattern, each being executable on
// a given CPU reference
// For instruction reference, see:  http://www.obelisk.me.uk/6502/reference.html
pub trait Instruction {
    fn execute(&self, cpu: &mut CPU);
}


#[cfg(test)]
mod test {
    use super::CPU;
    use crate::cpu::AddressingMode::*;

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

    #[test]
    fn accumulator_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0x8C;

        // Then
        assert_eq!(0x8C, cpu.read(&Accumulator));
    }

    #[test]
    fn accumulator_write() {
        // Given
        let mut cpu = CPU::new();

        // When
        cpu.write(&Accumulator, 0x07);

        // Then
        assert_eq!(0x07, cpu.accumulator);
    }

    #[test]
    fn absolute_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0xF9EA, 0x8A);

        // Then
        assert_eq!(0x8A, cpu.read(&Absolute(0xF9EA)));
    }

    #[test]
    fn absolute_write() {
        // Given
        let mut cpu = CPU::new();

        // When
        cpu.write(&Absolute(0xF9EA), 0x07);

        // Then
        assert_eq!(0x07, cpu.read_mem16(0xF9EA));
    }

    #[test]
    fn absolute_x_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x8D, 0x09);
        cpu.index_register_x = 0x0A;

        // Then
        assert_eq!(0x09, cpu.read(&AbsoluteX(0x83)));
    }

    #[test]
    fn absolute_x_no_wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x0104, 0x29);
        cpu.index_register_x = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&AbsoluteX(0x00FF)));
    }

    #[test]
    fn absolute_x_wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x0003, 0x29);
        cpu.index_register_x = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&AbsoluteX(0xFFFE)));
    }

    #[test]
    fn absolute_x_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x0A;

        // WHen
        cpu.write(&AbsoluteX(0x83), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x8D));
    }

    #[test]
    fn absolute_x_no_wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x07;

        // When
        cpu.write(&AbsoluteX(0x00FF), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x0106));
    }

    #[test]
    fn absolute_x_wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x07;

        // When
        cpu.write(&AbsoluteX(0xFFFD), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x0004));
    }

    #[test]
    fn absolute_y_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x8D, 0x09);
        cpu.index_register_y = 0x0A;

        // Then
        assert_eq!(0x09, cpu.read(&AbsoluteY(0x83)));
    }

    #[test]
    fn absolute_y_no_wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x0104, 0x29);
        cpu.index_register_y = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&AbsoluteY(0x00FF)));
    }

    #[test]
    fn absolute_y_wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x0003, 0x29);
        cpu.index_register_y = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&AbsoluteY(0xFFFE)));
    }

    #[test]
    fn absolute_y_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_y = 0x0A;

        // When
        cpu.write(&AbsoluteY(0x83), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x8D));
    }

    #[test]
    fn absolute_y_no_wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_y = 0x07;

        // When
        cpu.write(&AbsoluteY(0x00FF), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x0106));
    }

    #[test]
    fn absolute_y_wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_y = 0x07;

        // When
        cpu.write(&AbsoluteY(0xFFFD), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x0004));
    }

    #[test]
    fn immediate_read() {
        // Given
        let cpu = CPU::new();

        // Then
        assert_eq!(0xEA, cpu.read(&Immediate(0xEA)));
    }

    #[test]
    fn indirect_x_read() {
        // Example from Indexed indirect here:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x01;
        cpu.write_mem8(0x01, 0x05);
        cpu.write_mem8(0x02, 0x07);
        cpu.write_mem8(0x0705, 0x0A);

        // Then
        assert_eq!(0x0A, cpu.read(&IndirectX(0x00)));
    }

    #[test]
    fn indirect_x_write() {
        // Example derived from:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x01;
        cpu.write_mem8(0x01, 0x05);
        cpu.write_mem8(0x02, 0x07);

        // When
        cpu.write(&IndirectX(0x00), 0x0A);

        // Then
        assert_eq!(0x0A, cpu.read_mem8(0x0705));
    }


    #[test]
    fn indirect_y_read() {
        // Example from Indexed indirect here:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::new();

        cpu.index_register_y = 0x01;
        cpu.write_mem8(0x01, 0x03);
        cpu.write_mem8(0x02, 0x07);
        cpu.write_mem8(0x0704, 0x0A);

        // Then
        assert_eq!(0x0A, cpu.read(&IndirectY(0x01)));
    }

    #[test]
    fn indirect_y_write() {
        // Adapted from the indirect_y_read
        // Given
        let mut cpu = CPU::new();

        cpu.index_register_y = 0x01;
        cpu.write_mem8(0x01, 0x03);
        cpu.write_mem8(0x02, 0x07);

        // When
        cpu.write(&IndirectY(0x01), 0x0A);

        // Then
        assert_eq!(0x0A, cpu.read_mem8(0x0704));
    }

    #[test]
    fn zero_page_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0xA8, 0x0C);

        // Then
        assert_eq!(0x0C, cpu.read(&ZeroPage(0xA8)));
    }

    #[test]
    fn zero_page_write() {
        // Given
        let mut cpu = CPU::new();

        // When
        cpu.write(&ZeroPage(0xA8), 0xF1);

        // Then
        assert_eq!(0xF1, cpu.read_mem8(0xA8));
    }

    #[test]
    fn zero_page_x_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x8D, 0x09);
        cpu.index_register_x = 0x0A;

        // Then
        assert_eq!(0x09, cpu.read(&ZeroPageX(0x83)));
    }

    #[test]
    fn zero_page_x_wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x04, 0x29);
        cpu.index_register_x = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&ZeroPageX(0xFF)));
    }

    #[test]
    fn zero_page_x_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x0A;

        // When
        cpu.write(&ZeroPageX(0x83), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x8D));
    }

    #[test]
    fn zero_page_x_wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x07;

        // When
        cpu.write(&ZeroPageX(0xFF), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x06));
    }

    #[test]
    fn zero_page_y_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x8D, 0x09);
        cpu.index_register_y = 0x0A;

        // Then
        assert_eq!(0x09, cpu.read(&ZeroPageY(0x83)));
    }

    #[test]
    fn zero_page_y_wrap_around_read() {
        // Given
        let mut cpu = CPU::new();
        cpu.write_mem8(0x04, 0x29);
        cpu.index_register_y = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&ZeroPageY(0xFF)));
    }

    #[test]
    fn zero_page_y_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_y = 0x0A;

        // When
        cpu.write(&ZeroPageY(0x83), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x8D));
    }

    #[test]
    fn zero_page_y_wrap_around_write() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_y = 0x07;

        // When
        cpu.write(&ZeroPageY(0xFF), 0x09);

        // Then
        assert_eq!(0x09, cpu.read_mem8(0x06));
    }

}
