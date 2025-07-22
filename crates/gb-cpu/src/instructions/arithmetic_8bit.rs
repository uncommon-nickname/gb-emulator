// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::{MMU, MemoryAccess};

use crate::cpu::Cpu;
use crate::math::{add, decrement, increment};
use crate::registers::enums::{RegisterU8, RegisterU16};

macro_rules! make_inc_n8
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let old = cpu.registers.read_u8($reg);
                let new = increment(cpu, old);

                cpu.registers.write_u8($reg, new);

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

    let new = increment(cpu, byte);

    mmu.write_byte(addr, new);

    12
}

macro_rules! make_dec_n8
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let old = cpu.registers.read_u8($reg);
                let new = decrement(cpu, old);

                cpu.registers.write_u8($reg, new);

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

    let new = decrement(cpu, byte);

    mmu.write_byte(addr, new);

    12
}

macro_rules! make_add_u8
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let old = cpu.registers.read_u8(RegisterU8::A);
                let reg = cpu.registers.read_u8($reg);

                let new = add(cpu, old, reg);

                cpu.registers.write_u8(RegisterU8::A, new);

                4
            }
        )*
    };
}

make_add_u8! {
    add_a_b, RegisterU8::B;
    add_a_c, RegisterU8::C;
    add_a_d, RegisterU8::D;
    add_a_e, RegisterU8::E;
    add_a_h, RegisterU8::H;
    add_a_l, RegisterU8::L;
    add_a_a, RegisterU8::A;
}

pub fn add_a_hl(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let old = cpu.registers.read_u8(RegisterU8::A);
    let byte = mmu.read_byte(addr);

    let new = add(cpu, old, byte);

    cpu.registers.write_u8(RegisterU8::A, new);

    8
}
