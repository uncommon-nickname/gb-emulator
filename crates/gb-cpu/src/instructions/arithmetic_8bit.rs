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

macro_rules! make_dec_n8 {
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                cpu.registers.decrement_u8($reg);
                4
            }
        )*
    };
}

make_inc_n8! {
    inc_b_n8, RegisterU8::B;
    inc_d_n8, RegisterU8::D;
    inc_h_n8, RegisterU8::H;
    inc_c_n8, RegisterU8::C;
    inc_e_n8, RegisterU8::E;
    inc_l_n8, RegisterU8::L;
    inc_a_n8, RegisterU8::A;
}

make_dec_n8! {
    dec_b_n8, RegisterU8::B;
    dec_d_n8, RegisterU8::D;
    dec_h_n8, RegisterU8::H;
    dec_c_n8, RegisterU8::C;
    dec_e_n8, RegisterU8::E;
    dec_l_n8, RegisterU8::L;
    dec_a_n8, RegisterU8::A;
}
