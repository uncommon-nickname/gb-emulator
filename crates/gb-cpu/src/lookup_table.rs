// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::MMU;

use crate::cpu::Cpu;
use crate::instructions::arithmetic_8bit::*;
use crate::instructions::arithmetic_16bit::*;
use crate::instructions::control::*;
use crate::instructions::load_8bit::*;
use crate::instructions::load_16bit::*;

pub type OpCode = u8;
pub type Ticks = u32;
pub type InstructionFn = fn(OpCode, MMU<'_>, &mut Cpu) -> Ticks;

// NOTE: 18.07.2025
// I want this formatted in the same way that the opcode table
// is formatted for better readability.
//
// The index in this array corresponds to the opcode that should
// call the function defined in the array.
//
// How to read the opcode:
// Pick the number corresponding to the row (R) and the column (C).
// The opcode and index is 0xRC.
//
// Source: https://gbdev.io/gb-opcodes/optables/

#[rustfmt::skip]
pub const OPCODE_LOOKUP_TABLE: [InstructionFn; 256] = [
/*               x0          x1           x2           x3           x4           x5           x6           x7           x8           x9           xA           xB           xC           xD           xE           xF      */
/* 0x */    nop        , ld_bc_n16  , ld_bc_a    , inc_bc_n16 , inc_b_n8   , dec_b_n8   , unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, inc_c_n8   , dec_c_n8   , unsupported, unsupported,
/* 1x */    unsupported, ld_de_n16  , unsupported, inc_de_n16 , inc_d_n8   , dec_d_n8   , unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, inc_e_n8   , dec_e_n8   , unsupported, unsupported,
/* 2x */    unsupported, ld_hl_n16  , unsupported, inc_hl_n16 , inc_h_n8   , dec_h_n8   , unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, inc_l_n8   , dec_l_n8   , unsupported, unsupported,
/* 3x */    unsupported, ld_sp_n16  , unsupported, inc_sp_n16 , unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, inc_a_n8   , dec_a_n8   , unsupported, unsupported,
/* 4x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* 5x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* 6x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* 7x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* 8x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* 9x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* Ax */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* Bx */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* Cx */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* Dx */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* Ex */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* Fx */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
];
