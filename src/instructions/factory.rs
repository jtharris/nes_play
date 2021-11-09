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
    fn execute(&self, cpu: &mut CPU) {
        panic!("Unknown opcode encountered!  {}", self.opcode)
    }
}

fn generate_1byte_instruction(opcode: u8) -> InstructionExecution {
    match opcode {
        0x0A => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(ASL::new(Accumulator))
        },
        0x18 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(CLC{})
        },
        0x38 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(SEC{})
        },
        0x58 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(CLI{})
        },
        0x78 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(SEI{})
        },
        0xB8 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(CLV{})
        },
        0xD8 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(CLD{})
        },
        0xF8 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(SED{})
        },
        0x4A => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(LSR::new(Accumulator))
        },
        0xEA => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(NOP{})
        },
        0xAA => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(TAX{})
        },
        0x8A => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(TXA{})
        },
        0xCA => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(DEX{})
        },
        0xE8 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(INX{})
        },
        0xA8 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(TAY{})
        },
        0x98 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(TYA{})
        },
        0x88 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(DEY{})
        },
        0xC8 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(INY{})
        },
        0x2A => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(ROL::new(Accumulator))
        },
        0x6A => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(ROR::new(Accumulator))
        },
        0x40 => InstructionExecution {
            cycles: 6,
            page_boundary_cycle: false,
            instruction: Box::new(RTI{})
        },
        0x60 => InstructionExecution {
            cycles: 6,
            page_boundary_cycle: false,
            instruction: Box::new(RTS{})
        },
        0x9A => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(TXS{})
        },
        0xBA => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(TSX{})
        },
        0x48 => InstructionExecution {
            cycles: 3,
            page_boundary_cycle: false,
            instruction: Box::new(PHA{})
        },
        0x68 => InstructionExecution {
            cycles: 4,
            page_boundary_cycle: false,
            instruction: Box::new(PLA{})
        },
        0x08 => InstructionExecution {
            cycles: 3,
            page_boundary_cycle: false,
            instruction: Box::new(PHP{})
        },
        0x28 => InstructionExecution {
            cycles: 4,
            page_boundary_cycle: false,
            instruction: Box::new(PLP{})
        },
        _ => InstructionExecution {
            cycles: 0,
            page_boundary_cycle: false,
            instruction: Box::new(Unknown::new(opcode))
        }
    }
}

fn generate_2byte_instruction(opcode: u8, arg: u8) -> InstructionExecution {
    match opcode {
        0x69 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(ADC::new(Immediate(arg)))
        },
        0x65 => InstructionExecution {
            cycles: 3,
            page_boundary_cycle: false,
            instruction: Box::new(ADC::new(ZeroPage(arg)))
        },
        0x75 => InstructionExecution {
            cycles: 4,
            page_boundary_cycle: false,
            instruction: Box::new(ADC::new(ZeroPageX(arg)))
        },
        0x61 => InstructionExecution {
            cycles: 6,
            page_boundary_cycle: false,
            instruction: Box::new(ADC::new(IndirectX(arg)))
        },
        0x71 => InstructionExecution {
            cycles: 5,
            page_boundary_cycle: true,
            instruction: Box::new(ADC::new(IndirectY(arg)))
        },
        0x29 => InstructionExecution {
            cycles: 2,
            page_boundary_cycle: false,
            instruction: Box::new(AND::new(Immediate(arg)))
        },
        0x25 => InstructionExecution {
            cycles: 3,
            page_boundary_cycle: false,
            instruction: Box::new(AND::new(ZeroPage(arg)))
        },
        0x35 => InstructionExecution {
            cycles: 4,
            page_boundary_cycle: false,
            instruction: Box::new(AND::new(ZeroPageX(arg)))
        },
        0x21 => InstructionExecution {
            cycles: 6,
            page_boundary_cycle: false,
            instruction: Box::new(AND::new(IndirectX(arg)))
        },
        0x31 => InstructionExecution {
            cycles: 5,
            page_boundary_cycle: true,
            instruction: Box::new(AND::new(IndirectY(arg)))
        },
        0x06 => InstructionExecution {
            cycles: 5,
            page_boundary_cycle: false,
            instruction: Box::new(ASL::new(ZeroPage(arg)))
        },
        0x16 => InstructionExecution {
            cycles: 6,
            page_boundary_cycle: false,
            instruction: Box::new(ASL::new(ZeroPageX(arg)))
        },
        0x24 => InstructionExecution {
            cycles: 3,
            page_boundary_cycle: false,
            instruction: Box::new(BIT::new(ZeroPage(arg)))
        },

        _ => InstructionExecution {
            cycles: 0,
            page_boundary_cycle: false,
            instruction: Box::new(Unknown::new(opcode))
        }
    }
}

// TODO:  3 byte instructions