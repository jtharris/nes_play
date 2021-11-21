use crate::cpu::AddressingMode::{Accumulator, Immediate, ZeroPage, ZeroPageX, IndirectX, IndirectY, Absolute, AbsoluteX, AbsoluteY, ZeroPageY};
use crate::cpu::{CPU, Instruction};
use crate::instructions::adc::ADC;
use crate::instructions::and::AND;
use crate::instructions::asl::ASL;
use crate::instructions::bcc::BCC;
use crate::instructions::bcs::BCS;
use crate::instructions::beq::BEQ;
use crate::instructions::bit::BIT;
use crate::instructions::bmi::BMI;
use crate::instructions::bne::BNE;
use crate::instructions::bpl::BPL;
use crate::instructions::bvc::BVC;
use crate::instructions::bvs::BVS;
use crate::instructions::clc::CLC;
use crate::instructions::cld::CLD;
use crate::instructions::cli::CLI;
use crate::instructions::clv::CLV;
use crate::instructions::cmp::CMP;
use crate::instructions::cpx::CPX;
use crate::instructions::cpy::CPY;
use crate::instructions::dec::DEC;
use crate::instructions::dex::DEX;
use crate::instructions::dey::DEY;
use crate::instructions::eor::EOR;
use crate::instructions::inc::INC;
use crate::instructions::inx::INX;
use crate::instructions::iny::INY;
use crate::instructions::lda::LDA;
use crate::instructions::ldx::LDX;
use crate::instructions::ldy::LDY;
use crate::instructions::lsr::LSR;
use crate::instructions::nop::NOP;
use crate::instructions::ora::ORA;
use crate::instructions::pha::PHA;
use crate::instructions::php::PHP;
use crate::instructions::pla::PLA;
use crate::instructions::plp::PLP;
use crate::instructions::rol::ROL;
use crate::instructions::ror::ROR;
use crate::instructions::rti::RTI;
use crate::instructions::rts::RTS;
use crate::instructions::sbc::SBC;
use crate::instructions::sec::SEC;
use crate::instructions::sed::SED;
use crate::instructions::sei::SEI;
use crate::instructions::sta::STA;
use crate::instructions::stx::STX;
use crate::instructions::sty::STY;
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
    fn execute(&self, _: &mut CPU) -> u8 {
        panic!("Unknown opcode encountered!  {}", self.opcode);
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
        0x10 => Box::new(BPL::new(arg as i8)),
        0x30 => Box::new(BMI::new(arg as i8)),
        0x50 => Box::new(BVC::new(arg as i8)),
        0x70 => Box::new(BVS::new(arg as i8)),
        0x90 => Box::new(BCC::new(arg as i8)),
        0xB0 => Box::new(BCS::new(arg as i8)),
        0xD0 => Box::new(BNE::new(arg as i8)),
        0xF0 => Box::new(BEQ::new(arg as i8)),
        0xC9 => Box::new(CMP::new(Immediate(arg))),
        0xC5 => Box::new(CMP::new(ZeroPage(arg))),
        0xD5 => Box::new(CMP::new(ZeroPageX(arg))),
        0xC1 => Box::new(CMP::new(IndirectX(arg))),
        0xD1 => Box::new(CMP::new(IndirectY(arg))),
        0xE0 => Box::new(CPX::new(Immediate(arg))),
        0xE4 => Box::new(CPX::new(ZeroPage(arg))),
        0xC0 => Box::new(CPY::new(Immediate(arg))),
        0xC4 => Box::new(CPY::new(ZeroPage(arg))),
        0xC6 => Box::new(DEC::new(ZeroPage(arg))),
        0xD6 => Box::new(DEC::new(ZeroPageX(arg))),
        0x49 => Box::new(EOR::new(Immediate(arg))),
        0x45 => Box::new(EOR::new(ZeroPage(arg))),
        0x55 => Box::new(EOR::new(ZeroPageX(arg))),
        0x41 => Box::new(EOR::new(IndirectX(arg))),
        0x51 => Box::new(EOR::new(IndirectY(arg))),
        0xE6 => Box::new(INC::new(ZeroPage(arg))),
        0xF6 => Box::new(INC::new(ZeroPageX(arg))),
        0xA9 => Box::new(LDA::new(Immediate(arg))),
        0xA5 => Box::new(LDA::new(ZeroPage(arg))),
        0xB5 => Box::new(LDA::new(ZeroPageX(arg))),
        0xA1 => Box::new(LDA::new(IndirectX(arg))),
        0xB1 => Box::new(LDA::new(IndirectY(arg))),
        0xA2 => Box::new(LDX::new(Immediate(arg))),
        0xA6 => Box::new(LDX::new(ZeroPage(arg))),
        0xB6 => Box::new(LDX::new(ZeroPageY(arg))),
        0xA0 => Box::new(LDY::new(Immediate(arg))),
        0xA4 => Box::new(LDY::new(ZeroPage(arg))),
        0xB4 => Box::new(LDY::new(ZeroPageX(arg))),
        0x46 => Box::new(LSR::new(ZeroPage(arg))),
        0x56 => Box::new(LSR::new(ZeroPageX(arg))),
        0x09 => Box::new(ORA::new(Immediate(arg))),
        0x05 => Box::new(ORA::new(ZeroPage(arg))),
        0x15 => Box::new(ORA::new(ZeroPageX(arg))),
        0x01 => Box::new(ORA::new(IndirectX(arg))),
        0x11 => Box::new(ORA::new(IndirectY(arg))),
        0x26 => Box::new(ROL::new(ZeroPage(arg))),
        0x36 => Box::new(ROL::new(ZeroPageX(arg))),
        0x66 => Box::new(ROR::new(ZeroPage(arg))),
        0x76 => Box::new(ROR::new(ZeroPageX(arg))),
        0xE9 => Box::new(SBC::new(Immediate(arg))),
        0xE5 => Box::new(SBC::new(ZeroPage(arg))),
        0xF5 => Box::new(SBC::new(ZeroPageX(arg))),
        0xE1 => Box::new(SBC::new(IndirectX(arg))),
        0xF1 => Box::new(SBC::new(IndirectY(arg))),
        0x85 => Box::new(STA::new(ZeroPage(arg))),
        0x95 => Box::new(STA::new(ZeroPageX(arg))),
        0x81 => Box::new(STA::new(IndirectX(arg))),
        0x91 => Box::new(STA::new(IndirectY(arg))),
        0x86 => Box::new(STX::new(ZeroPage(arg))),
        0x96 => Box::new(STX::new(ZeroPageY(arg))),
        0x84 => Box::new(STY::new(ZeroPage(arg))),
        0x94 => Box::new(STY::new(ZeroPageX(arg))),

        _ => Box::new(Unknown::new(opcode))
    }
}

// TODO:  3 byte instructions
fn generate_3byte_instruction(opcode: u8, arg: u16) -> Box<dyn Instruction> {
    match opcode {
        0x6D => Box::new(ADC::new(Absolute(arg))),
        0x7D => Box::new(ADC::new(AbsoluteX(arg))),
        0x79 => Box::new(ADC::new(AbsoluteY(arg))),
        0x2D => Box::new(AND::new(Absolute(arg))),
        0x3D => Box::new(AND::new(AbsoluteX(arg))),
        0x39 => Box::new(AND::new(AbsoluteY(arg))),
        0x0E => Box::new(ASL::new(Absolute(arg))),
        0x1E => Box::new(ASL::new(AbsoluteX(arg))),
        0x2C => Box::new(BIT::new(Absolute(arg))),

        // TODO:  Lots more to go

        _ => Box::new(Unknown::new(opcode))
    }
}