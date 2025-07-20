// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::{MMU, MemoryAccess};

use crate::cpu::Cpu;
use crate::registers::enums::{RegisterU8, RegisterU16};

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
    inc_b, RegisterU8::B;
    inc_d, RegisterU8::D;
    inc_h, RegisterU8::H;
    inc_c, RegisterU8::C;
    inc_e, RegisterU8::E;
    inc_l, RegisterU8::L;
    inc_a, RegisterU8::A;
}

pub fn inci_hl(_opcode: u8, mut mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let byte = mmu.read_byte(addr);

    let new = cpu.registers.increment(byte);
    mmu.write_byte(addr, new);

    12
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

make_dec_n8! {
    dec_b, RegisterU8::B;
    dec_d, RegisterU8::D;
    dec_h, RegisterU8::H;
    dec_c, RegisterU8::C;
    dec_e, RegisterU8::E;
    dec_l, RegisterU8::L;
    dec_a, RegisterU8::A;
}

pub fn decd_hl(_opcode: u8, mut mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let byte = mmu.read_byte(addr);

    let new = cpu.registers.decrement(byte);
    mmu.write_byte(addr, new);

    12
}
