// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::MMU;

use crate::cpu::Cpu;
use crate::registers::enums::RegisterU8;

macro_rules! make_inc_n8 {
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                cpu.registers.increment_u8($reg);
                4
            }
        )*
    };
}

make_inc_n8! {
    inc_b_n8, RegisterU8::B;
}
