use std::fmt;
use std::fmt::{Formatter, write};
use std::io::Write;
use crate::bus::Bus;
use crate::instructions::factory::generate_instruction;

// http://wiki.nesdev.com/w/index.php/CPU_registers
pub struct CPU {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub index_register_x: u8,
    pub index_register_y: u8,
    pub processor_status: u8,  // http://wiki.nesdev.com/w/index.php/Status_flags
    bus: Bus
}

impl CPU {
    pub fn empty() -> Self {
        let mut cpu = CPU::new([0; 0x4000]);
        cpu.processor_status = 0;   // Zero out the PS

        cpu
    }

    pub fn new(program: [u8; 0x4000]) -> Self {
        //http://wiki.nesdev.com/w/index.php/CPU_power_up_state
        CPU {
            program_counter: 0xC000,  // TODO:  This is standard?
            stack_pointer: 0xFD,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: 0x24,  // This is from the nestest golden log...
            bus: Bus::new(program)
        }
    }

    // http://wiki.nesdev.com/w/index.php/Status_flags
    pub fn set_flag(&mut self, flag: StatusFlag, value: bool) {
        self.processor_status = match value {
            true => self.processor_status | flag.bitmask(),
            false => self.processor_status & !flag.bitmask(),
        };
    }

    pub fn get_flag(&self, flag: StatusFlag) -> bool {
        let bitmask = flag.bitmask();
        self.processor_status & bitmask == bitmask
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
                self.bus.read_mem16(base.wrapping_add(self.index_register_x) as u16)
            }
            &AddressingMode::IndirectY(address) => {
                let bytes = [
                    self.bus.read_mem8(address as u16),
                    self.bus.read_mem8(address.wrapping_add(1) as u16)
                ];
                let value = u16::from_le_bytes(bytes);

                value.wrapping_add(self.index_register_y as u16)
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

    pub fn default_cycles(&self, mode: &AddressingMode) -> u8 {
        let base_cycles = match mode {
            &AddressingMode::Accumulator => 0,
            &AddressingMode::Immediate(_) => 2,
            &AddressingMode::ZeroPage(_) => 3,
            &AddressingMode::ZeroPageX(_) => 4,
            &AddressingMode::ZeroPageY(_) => 4,
            &AddressingMode::Absolute(_) => 4,
            &AddressingMode::AbsoluteX(base) => 4,
            &AddressingMode::AbsoluteY(base) => 4,
            &AddressingMode::IndirectX(_) => 6,
            &AddressingMode::IndirectY(address) => 5,
        };

        base_cycles + self.extra_cycles(mode)
    }

    fn extra_cycles(&self, mode: &AddressingMode) -> u8 {
        match mode {
            &AddressingMode::AbsoluteX(_) | 
            &AddressingMode::AbsoluteY(_) | 
            &AddressingMode::IndirectY(_) => if self.mem_address(mode) > 0xFF { 1 } else { 0 },
            _ => 0
        }
    }

    pub fn memory_cycles(&self, mode: &AddressingMode) -> u8 {
        match mode {
            &AddressingMode::Accumulator => 2,
            &AddressingMode::ZeroPage(_) => 5,
            &AddressingMode::ZeroPageX(_) => 6,
            &AddressingMode::Absolute(_) => 6,
            &AddressingMode::AbsoluteX(_) => 7,
            _ => panic!("Invalid addressing mode for memory cycles")
        }
    }

    pub fn read(&self, mode: &AddressingMode) -> u8 {
        match mode {
            &AddressingMode::Accumulator => self.accumulator,
            &AddressingMode::Immediate(value) => value,
            am => self.bus.read_mem8(self.mem_address(am))
        }
    }

    pub fn write(&mut self, mode: &AddressingMode, value: u8) {
        match mode {
            &AddressingMode::Accumulator => self.accumulator = value,
            &AddressingMode::Immediate(_) => {},
            am => self.bus.write_mem8(self.mem_address(am), value)
        }
    }

    // See Stack Operations http://obelisk.me.uk/6502/instructions.html
    pub fn push_stack(&mut self, value: u8) {
        self.bus.write_mem8(self.stack_pointer as u16 | 0x0100, value);
        self.stack_pointer -= 1;
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.bus.read_mem8(self.stack_pointer as u16 | 0x0100)
    }

    pub fn read_mem16(&self, addr: u16) -> u16 {
        self.bus.read_mem16(addr)
    }

    pub fn write_mem16(&mut self, addr: u16, data: u16) {
        self.bus.write_mem16(addr, data)
    }

    // Returns a 1 if page boundary was crossed, 0 otherwise
    pub fn set_pc(&mut self, value: u16) -> u8 {
        let old_pc = self.program_counter;
        self.program_counter = value;

        if old_pc >> 8 == value >> 8 {
            0
        } else {
            1
        }
    }

    pub fn log_execution(&mut self, mut log: Box<dyn Write>) {
        let mut cycle: usize = 4;  // TODO:  Make this configurable?

        loop {
            let pc = self.program_counter;
            let instruction = generate_instruction(self);

            write!(log, "{:04X} ", pc);
            write!(log, "         ");    // TODO:  Fill in bytes later
            match &instruction {
                Some(inst) => write!(log, "{:<25}", inst.to_string()),
                None => write!(log, "{:<25}", "No Instruction")
            };
            write!(log, "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} ",
                   self.accumulator, self.index_register_x, self.index_register_y,
                   self.processor_status, self.stack_pointer);
            write!(log, "PPU:  0, 0 ");  // TODO:  Figure this out too

            match &instruction {
                Some(inst) => {
                    let new_cycles = inst.execute(self);
                    cycle += new_cycles as usize;
                },
                None => return
            };

            writeln!(log, "CYC:{}", cycle);
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

impl StatusFlag {
    fn bitmask(&self) -> u8 {
        match self {
            StatusFlag::Carry =>            0b0000_0001,
            StatusFlag::Zero =>             0b0000_0010,
            StatusFlag::InterruptDisable => 0b0000_0100,
            StatusFlag::Decimal =>          0b0000_1000,
            StatusFlag::Overflow =>         0b0100_0000,
            StatusFlag::Negative =>         0b1000_0000
        }
    }
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

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AddressingMode::Accumulator => write!(f, "A"),
            AddressingMode::Absolute(addr) => write!(f, "${:04X}", addr),
            AddressingMode::AbsoluteX(addr) => write!(f, "${:04X},X", addr),
            AddressingMode::AbsoluteY(addr) => write!(f, "${:04X},Y", addr),
            AddressingMode::Immediate(val) => write!(f, "#${:02X}", val),
            AddressingMode::IndirectX(addr) => write!(f, "(${:02X},X)", addr),
            AddressingMode::IndirectY(addr) => write!(f, "(${:02X}),Y", addr),
            AddressingMode::ZeroPage(addr) => write!(f, "${:02X}", addr),
            AddressingMode::ZeroPageX(addr) => write!(f, "${:02X},X", addr),
            AddressingMode::ZeroPageY(addr) => write!(f, "${:02X},Y", addr),
        }
    }
}


// Instructions are implemented as a visitor pattern, each being executable on
// a given CPU reference
// For instruction reference, see:  http://www.obelisk.me.uk/6502/reference.html
// Return value is the number of machine cycles take to execute
pub trait Instruction: fmt::Display {
    fn execute(&self, cpu: &mut CPU) -> u8;
}


#[cfg(test)]
mod test {
    use super::CPU;
    use crate::cpu::AddressingMode::*;
    use crate::cpu::StatusFlag;

    #[test]
    fn read_write_16bit_memory() {
        // Given
        let mut cpu = CPU::empty();
        cpu.write_mem16(0x0800, 0xFF9B);

        // Then
        assert_eq!(0xFF9B, cpu.read_mem16(0x0800));
    }

    #[test]
    fn accumulator_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x8C;

        // Then
        assert_eq!(0x8C, cpu.read(&Accumulator));
    }

    #[test]
    fn accumulator_write() {
        // Given
        let mut cpu = CPU::empty();

        // When
        cpu.write(&Accumulator, 0x07);

        // Then
        assert_eq!(0x07, cpu.accumulator);
    }

    #[test]
    fn absolute_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x03EA, 0x8A);

        // Then
        assert_eq!(0x8A, cpu.read(&Absolute(0x03EA)));
    }

    #[test]
    fn absolute_write() {
        // Given
        let mut cpu = CPU::empty();

        // When
        cpu.write(&Absolute(0x02E6), 0x07);

        // Then
        assert_eq!(0x07, cpu.read_mem16(0x02E6));
    }

    #[test]
    fn absolute_x_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x8D, 0x09);
        cpu.index_register_x = 0x0A;

        // Then
        assert_eq!(0x09, cpu.read(&AbsoluteX(0x83)));
    }

    #[test]
    fn absolute_x_no_wrap_around_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x0104, 0x29);
        cpu.index_register_x = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&AbsoluteX(0x00FF)));
    }

    #[test]
    fn absolute_x_wrap_around_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x0003, 0x29);
        cpu.index_register_x = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&AbsoluteX(0xFFFE)));
    }

    #[test]
    fn absolute_x_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x0A;

        // WHen
        cpu.write(&AbsoluteX(0x83), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x8D));
    }

    #[test]
    fn absolute_x_no_wrap_around_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x07;

        // When
        cpu.write(&AbsoluteX(0x00FF), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x0106));
    }

    #[test]
    fn absolute_x_wrap_around_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x07;

        // When
        cpu.write(&AbsoluteX(0xFFFD), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x0004));
    }

    #[test]
    fn absolute_y_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x8D, 0x09);
        cpu.index_register_y = 0x0A;

        // Then
        assert_eq!(0x09, cpu.read(&AbsoluteY(0x83)));
    }

    #[test]
    fn absolute_y_no_wrap_around_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x0104, 0x29);
        cpu.index_register_y = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&AbsoluteY(0x00FF)));
    }

    #[test]
    fn absolute_y_wrap_around_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x0003, 0x29);
        cpu.index_register_y = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&AbsoluteY(0xFFFE)));
    }

    #[test]
    fn absolute_y_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x0A;

        // When
        cpu.write(&AbsoluteY(0x83), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x8D));
    }

    #[test]
    fn absolute_y_no_wrap_around_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x07;

        // When
        cpu.write(&AbsoluteY(0x00FF), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x0106));
    }

    #[test]
    fn absolute_y_wrap_around_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x07;

        // When
        cpu.write(&AbsoluteY(0xFFFD), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x0004));
    }

    #[test]
    fn immediate_read() {
        // Given
        let cpu = CPU::empty();

        // Then
        assert_eq!(0xEA, cpu.read(&Immediate(0xEA)));
    }

    #[test]
    fn indirect_x_read() {
        // Example from Indexed indirect here:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x01;
        cpu.bus.write_mem8(0x01, 0x05);
        cpu.bus.write_mem8(0x02, 0x07);
        cpu.bus.write_mem8(0x0705, 0x0A);

        // Then
        assert_eq!(0x0A, cpu.read(&IndirectX(0x00)));
    }

    #[test]
    fn indirect_x_write() {
        // Example derived from:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x01;
        cpu.bus.write_mem8(0x01, 0x05);
        cpu.bus.write_mem8(0x02, 0x07);

        // When
        cpu.write(&IndirectX(0x00), 0x0A);

        // Then
        assert_eq!(0x0A, cpu.bus.read_mem8(0x0705));
    }

    #[test]
    fn indirect_y_read() {
        // Example from Indexed indirect here:  https://skilldrick.github.io/easy6502/#addressing
        // Given
        let mut cpu = CPU::empty();

        cpu.index_register_y = 0x01;
        cpu.bus.write_mem8(0x01, 0x03);
        cpu.bus.write_mem8(0x02, 0x07);
        cpu.bus.write_mem8(0x0704, 0x0A);

        // Then
        assert_eq!(0x0A, cpu.read(&IndirectY(0x01)));
    }

    // https://stackoverflow.com/questions/46262435/indirect-y-indexed-addressing-mode-in-mos-6502
    #[test]
    fn indirect_y_read_with_carry() {
        let mut cpu = CPU::empty();

        cpu.index_register_y = 0x90;
        cpu.bus.write_mem8(0x02, 0x80);
        cpu.bus.write_mem8(0x03, 0x02);
        cpu.bus.write_mem8(0x0310, 0xCA);

        // Then
        assert_eq!(0xCA, cpu.read(&IndirectY(0x02)));
    }

    #[test]
    fn indirect_y_write() {
        // Adapted from the indirect_y_read
        // Given
        let mut cpu = CPU::empty();

        cpu.index_register_y = 0x01;
        cpu.bus.write_mem8(0x01, 0x03);
        cpu.bus.write_mem8(0x02, 0x07);

        // When
        cpu.write(&IndirectY(0x01), 0x0A);

        // Then
        assert_eq!(0x0A, cpu.bus.read_mem8(0x0704));
    }

    #[test]
    fn zero_page_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0xA8, 0x0C);

        // Then
        assert_eq!(0x0C, cpu.read(&ZeroPage(0xA8)));
    }

    #[test]
    fn zero_page_write() {
        // Given
        let mut cpu = CPU::empty();

        // When
        cpu.write(&ZeroPage(0xA8), 0xF1);

        // Then
        assert_eq!(0xF1, cpu.bus.read_mem8(0xA8));
    }

    #[test]
    fn zero_page_x_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x8D, 0x09);
        cpu.index_register_x = 0x0A;

        // Then
        assert_eq!(0x09, cpu.read(&ZeroPageX(0x83)));
    }

    #[test]
    fn zero_page_x_wrap_around_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x04, 0x29);
        cpu.index_register_x = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&ZeroPageX(0xFF)));
    }

    #[test]
    fn zero_page_x_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x0A;

        // When
        cpu.write(&ZeroPageX(0x83), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x8D));
    }

    #[test]
    fn zero_page_x_wrap_around_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x07;

        // When
        cpu.write(&ZeroPageX(0xFF), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x06));
    }

    #[test]
    fn zero_page_y_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x8D, 0x09);
        cpu.index_register_y = 0x0A;

        // Then
        assert_eq!(0x09, cpu.read(&ZeroPageY(0x83)));
    }

    #[test]
    fn zero_page_y_wrap_around_read() {
        // Given
        let mut cpu = CPU::empty();
        cpu.bus.write_mem8(0x04, 0x29);
        cpu.index_register_y = 0x05;

        // Then
        assert_eq!(0x29, cpu.read(&ZeroPageY(0xFF)));
    }

    #[test]
    fn zero_page_y_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x0A;

        // When
        cpu.write(&ZeroPageY(0x83), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x8D));
    }

    #[test]
    fn zero_page_y_wrap_around_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x07;

        // When
        cpu.write(&ZeroPageY(0xFF), 0x09);

        // Then
        assert_eq!(0x09, cpu.bus.read_mem8(0x06));
    }

    #[test]
    fn push_values_to_stack() {
        // Given
        let mut cpu = CPU::empty();

        // When
        cpu.push_stack(0x83);
        cpu.push_stack(0x04);
        cpu.push_stack(0xF3);

        // Then
        assert_eq!(0xFA, cpu.stack_pointer);
        assert_eq!(0x83, cpu.bus.read_mem8(0x01FD));
        assert_eq!(0x04, cpu.bus.read_mem8(0x01FC));
        assert_eq!(0xF3, cpu.bus.read_mem8(0x01FB));
    }

    #[test]
    fn push_and_pop_stack() {
        // Given
        let mut cpu = CPU::empty();

        // When
        cpu.push_stack(0x83);
        cpu.push_stack(0x04);

        // Then
        assert_eq!(0x04, cpu.pop_stack());
        assert_eq!(0xFC, cpu.stack_pointer);

        assert_eq!(0x83, cpu.pop_stack());
        assert_eq!(0xFD, cpu.stack_pointer);
    }

}
