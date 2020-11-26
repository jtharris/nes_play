use crate::cpu::{CPU, AddressingMode};

pub struct Immediate {
    value: u8
}

impl Immediate {
    pub fn new(value: u8) -> Self {
        Immediate{ value }
    }
}

impl AddressingMode for Immediate {
    fn read(&self, _: &CPU) -> u8 {
        self.value
    }

    fn write(&self, _: &mut CPU, _: u8) {
        // do nothing sentence
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode};
    use super::Immediate;

    #[test]
    fn basic_read() {
        let addressing_mode = Immediate::new(0xEA);

        assert_eq!(0xEA, addressing_mode.read(&CPU::new()));
    }

}