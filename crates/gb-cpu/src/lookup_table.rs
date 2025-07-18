// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::MMU;

use crate::cpu::Cpu;
use crate::instructions::control::instr_nop;

type OpCode = u8;
type Ticks = u32;
type InstructionFn = fn(OpCode, &mut MMU<'_>, &mut Cpu) -> Ticks;

pub const LOOKUP_TABLE: [InstructionFn; 1] = [
    //  0
    instr_nop, // 0
];
