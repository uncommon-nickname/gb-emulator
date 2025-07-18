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
#[rustfmt::skip]
pub const LOOKUP_TABLE: [InstructionFn; 5] = [
    nop, ld_bc_n16, ld_bc_a, inc_bc_n16, inc_b_n8,
];
