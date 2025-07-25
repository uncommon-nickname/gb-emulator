// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::MMU;

use crate::cpu::Cpu;
use crate::registers::enums::RegisterU16;

macro_rules! make_ld_n16
{
    ($($name:ident, $reg:expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let val = cpu.read_pc_word(&mmu);
                cpu.registers.write_u16($reg, val);
                12
            }
        )*
    };
}

make_ld_n16! {
    ld_bc_n16, RegisterU16::BC;
    ld_de_n16, RegisterU16::DE;
    ld_hl_n16, RegisterU16::HL;
    ld_sp_n16, RegisterU16::SP;
}
