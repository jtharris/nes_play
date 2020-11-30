use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#ADC
struct ADC {
    mode: AddressingMode
}

impl ADC {
    pub fn new(mode: AddressingMode) -> Self {
        ADC{ mode }
    }
}

impl Instruction for ADC {
    fn execute(&self, cpu: &mut CPU) {
        let memory_value= cpu.read(&self.mode);

        // TODO:  Be sure to add one if carry flag is set

        let (sum, overflow) = cpu.accumulator.overflowing_add(memory_value);
        let (signed_sum, signed_overflow) = (cpu.accumulator as i8).overflowing_add(memory_value as i8);

        cpu.accumulator = sum;
        cpu.set_flag(StatusFlag::Carry, overflow);
        cpu.set_flag(StatusFlag::Zero, sum == 0);
        cpu.set_flag(StatusFlag::Overflow, signed_overflow);
        cpu.set_flag(StatusFlag::Negative, signed_sum < 0);
    }
}

#[cfg(test)]
mod test {


}