// http://wiki.nesdev.com/w/index.php/CPU_registers
pub struct CPU {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub index_register_x: u8,
    pub index_register_y: u8,
    pub processor_status: u8,  // http://wiki.nesdev.com/w/index.php/Status_flags
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            processor_status: 0
        }
    }
}

// Instructions are implemented as a visitor pattern, each being executable on
// a given CPU reference
// For instruction reference, see:  http://www.obelisk.me.uk/6502/reference.html
pub trait Instruction {
    fn execute(&self, cpu: &mut CPU);
}
