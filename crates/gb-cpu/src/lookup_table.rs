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
/* 0x */    nop        , ld_bc_n16  , ld_bc_a    , inc_bc     , inc_b      , dec_b      , ld_b_n8    , unsupported, unsupported, unsupported, ld_a_bc    , unsupported, inc_c      , dec_c      , ld_c_n8    , unsupported,
/* 1x */    unsupported, ld_de_n16  , ld_de_a    , inc_de     , inc_d      , dec_d      , ld_d_n8    , unsupported, unsupported, unsupported, ld_a_de    , unsupported, inc_e      , dec_e      , ld_e_n8    , unsupported,
/* 2x */    unsupported, ld_hl_n16  , ldi_hl_a   , inc_hl     , inc_h      , dec_h      , ld_h_n8    , unsupported, unsupported, unsupported, ldi_a_hl   , unsupported, inc_l      , dec_l      , ld_l_n8    , unsupported,
/* 3x */    unsupported, ld_sp_n16  , ldd_hl_a   , inc_sp     , inci_hl    , decd_hl    , ld_hl_n8   , unsupported, unsupported, unsupported, ldd_a_hl   , unsupported, inc_a      , dec_a      , ld_a_n8    , unsupported,
/* 4x */    ld_b_b     , ld_b_c     , ld_b_d     , ld_b_e     , ld_b_h     , ld_b_l     , ld_b_hl    , ld_b_a     , ld_c_b     , ld_c_c     , ld_c_d     , ld_c_e     , ld_c_h     , ld_c_l     , ld_c_hl    , ld_c_a     ,
/* 5x */    ld_d_b     , ld_d_c     , ld_d_d     , ld_d_e     , ld_d_h     , ld_d_l     , ld_d_hl    , ld_d_a     , ld_e_b     , ld_e_c     , ld_e_d     , ld_e_e     , ld_e_h     , ld_e_l     , ld_e_hl    , ld_e_a     ,
/* 6x */    ld_h_b     , ld_h_c     , ld_h_d     , ld_h_e     , ld_h_h     , ld_h_l     , ld_h_hl    , ld_h_a     , ld_l_b     , ld_l_c     , ld_l_d     , ld_l_e     , ld_l_h     , ld_l_l     , ld_l_hl    , ld_l_a     ,
/* 7x */    ld_hl_b    , ld_hl_c    , ld_hl_d    , ld_hl_e    , ld_hl_h    , ld_hl_l    , halt       , ld_hl_a    , ld_a_b     , ld_a_c     , ld_a_d     , ld_a_e     , ld_a_h     , ld_a_l     , ld_a_hl    , ld_a_a     ,
/* 8x */    add_a_b    , add_a_c    , add_a_d    , add_a_e    , add_a_h    , add_a_l    , add_a_hl   , add_a_a    , adc_a_b    , adc_a_c    , adc_a_d    , adc_a_e    , adc_a_h    , adc_a_l    , adc_a_hl   , adc_a_a    ,
/* 9x */    sub_a_b    , sub_a_c    , sub_a_d    , sub_a_e    , sub_a_h    , sub_a_l    , sub_a_hl   , sub_a_a    , sbc_a_b    , sbc_a_c    , sbc_a_d    , sbc_a_e    , sbc_a_h    , sbc_a_l    , sbc_a_hl   , sbc_a_a    ,
/* Ax */    and_a_b    , and_a_c    , and_a_d    , and_a_e    , and_a_h    , and_a_l    , and_a_hl   , and_a_a    , xor_a_b    , xor_a_c    , xor_a_d    , xor_a_e    , xor_a_h    , xor_a_l    , xor_a_hl   , xor_a_a    ,
/* Bx */    or_a_b     , or_a_c     , or_a_d     , or_a_e     , or_a_h     , or_a_l     , or_a_hl    , or_a_a     , cp_a_b     , cp_a_c     , cp_a_d     , cp_a_e     , cp_a_h     , cp_a_l     , cp_a_hl    , cp_a_a     ,
/* Cx */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, add_a_n8   , unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, adc_a_n8   , unsupported,
/* Dx */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, sub_a_n8   , unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, sbc_a_n8   , unsupported,
/* Ex */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, and_a_n8   , unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, xor_a_n8   , unsupported,
/* Fx */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, or_a_n8    , unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, cp_a_n8    , unsupported,
];

#[allow(unused)]
#[rustfmt::skip]
pub const CB_LOOKUP_TABLE: [InstructionFn; 256] = [
/*               x0          x1           x2           x3           x4           x5           x6           x7           x8           x9           xA           xB           xC           xD           xE           xF      */
/* 0x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* 1x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* 2x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
/* 3x */    unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported, unsupported,
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
