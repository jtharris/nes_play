use crate::cpu::AddressingMode::{Accumulator, Immediate, ZeroPage, ZeroPageX, IndirectX, IndirectY};
use crate::cpu::{CPU, Instruction};
use crate::instructions::adc::ADC;
use crate::instructions::and::AND;
use crate::instructions::bit::BIT;
use crate::instructions::clc::CLC;
use crate::instructions::asl::ASL;
use crate::instructions::cld::CLD;
use crate::instructions::cli::CLI;
use crate::instructions::clv::CLV;
use crate::instructions::dex::DEX;
use crate::instructions::dey::DEY;
use crate::instructions::inx::INX;
use crate::instructions::iny::INY;
use crate::instructions::lsr::LSR;
use crate::instructions::nop::NOP;
use crate::instructions::pha::PHA;
use crate::instructions::php::PHP;
use crate::instructions::pla::PLA;
use crate::instructions::plp::PLP;
use crate::instructions::rol::ROL;
use crate::instructions::ror::ROR;
use crate::instructions::rti::RTI;
use crate::instructions::rts::RTS;
use crate::instructions::sec::SEC;
use crate::instructions::sed::SED;
use crate::instructions::sei::SEI;
use crate::instructions::tax::TAX;
use crate::instructions::tay::TAY;
use crate::instructions::tsx::TSX;
use crate::instructions::txa::TXA;
use crate::instructions::txs::TXS;
use crate::instructions::tya::TYA;

struct InstructionExecution {
    pub cycles: u8,
    pub page_boundary_cycle: bool,
    pub instruction: Box<dyn Instruction>
}

struct Unknown {
    opcode: u8
}

impl Unknown {
    pub fn new (opcode: u8) -> Self {
        Unknown { opcode }
    }
}

impl Instruction for Unknown {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        panic!("Unknown opcode encountered!  {}", self.opcode);
        0
    }
}

fn generate_1byte_instruction(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x0A => Box::new(ASL::new(Accumulator)),
        0x18 => Box::new(CLC{}),
        0x38 => Box::new(SEC{}),
        0x58 => Box::new(CLI{}),
        0x78 => Box::new(SEI{}),
        0xB8 => Box::new(CLV{}),
        0xD8 => Box::new(CLD{}),
        0xF8 => Box::new(SED{}),
        0x4A => Box::new(LSR::new(Accumulator)),
        0xEA => Box::new(NOP{}),
        0xAA => Box::new(TAX{}),
        0x8A => Box::new(TXA{}),
        0xCA => Box::new(DEX{}),
        0xE8 => Box::new(INX{}),
        0xA8 => Box::new(TAY{}),
        0x98 => Box::new(TYA{}),
        0x88 => Box::new(DEY{}),
        0xC8 => Box::new(INY{}),
        0x2A => Box::new(ROL::new(Accumulator)),
        0x6A => Box::new(ROR::new(Accumulator)),
        0x40 => Box::new(RTI{}),
        0x60 => Box::new(RTS{}),
        0x9A => Box::new(TXS{}),
        0xBA => Box::new(TSX{}),
        0x48 => Box::new(PHA{}),
        0x68 => Box::new(PLA{}),
        0x08 => Box::new(PHP{}),
        0x28 => Box::new(PLP{}),
        _ => Box::new(Unknown::new(opcode))
    }
}

fn generate_2byte_instruction(opcode: u8, arg: u8) -> Box<dyn Instruction> {
    match opcode {
        0x69 => Box::new(ADC::new(Immediate(arg))),
        0x65 => Box::new(ADC::new(ZeroPage(arg))),
        0x75 => Box::new(ADC::new(ZeroPageX(arg))),
        0x61 => Box::new(ADC::new(IndirectX(arg))),
        0x71 => Box::new(ADC::new(IndirectY(arg))),
        0x29 => Box::new(AND::new(Immediate(arg))),
        0x25 => Box::new(AND::new(ZeroPage(arg))),
        0x35 => Box::new(AND::new(ZeroPageX(arg))),
        0x21 => Box::new(AND::new(IndirectX(arg))),
        0x31 => Box::new(AND::new(IndirectY(arg))),
        0x06 => Box::new(ASL::new(ZeroPage(arg))),
        0x16 => Box::new(ASL::new(ZeroPageX(arg))),
        0x24 => Box::new(BIT::new(ZeroPage(arg))),

        _ => Box::new(Unknown::new(opcode))
    }
}

// TODO:  3 byte instructions