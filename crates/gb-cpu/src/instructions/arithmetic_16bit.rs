// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::MMU;

use crate::cpu::Cpu;
use crate::registers::enums::RegisterU16;

macro_rules! make_inc_u16
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let val = cpu.registers.read_u16($reg);
                cpu.registers.write_u16($reg, val.wrapping_add(1));
                8
            }
        )*
    };
}

make_inc_u16! {
    inc_bc, RegisterU16::BC;
    inc_de, RegisterU16::DE;
    inc_hl, RegisterU16::HL;
    inc_sp, RegisterU16::SP;
}
