// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::{MMU, MemoryAccess};

use crate::cpu::Cpu;
use crate::registers::enums::{RegisterU8, RegisterU16};

macro_rules! make_inc_n8
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let old = cpu.registers.read_u8($reg);
                let new = cpu.increment(old);

                cpu.registers.write_u8($reg, new);

                4
            }
        )*
    };
}

macro_rules! make_dec_n8
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let old = cpu.registers.read_u8($reg);
                let new = cpu.decrement(old);

                cpu.registers.write_u8($reg, new);

                4
            }
        )*
    };
}

macro_rules! make_add_u8
{
    ($($name:ident, $reg: expr, $consider_carry: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let old = cpu.registers.read_u8(RegisterU8::A);
                let reg = cpu.registers.read_u8($reg);

                let new = cpu.add(old, reg, $consider_carry);

                cpu.registers.write_u8(RegisterU8::A, new);

                4
            }
        )*
    };
}

macro_rules! make_sub_u8
{
    ($($name:ident, $reg: expr, $consider_carry: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let old = cpu.registers.read_u8(RegisterU8::A);
                let reg = cpu.registers.read_u8($reg);

                let new = cpu.sub(old, reg, $consider_carry);

                cpu.registers.write_u8(RegisterU8::A, new);

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

    let new = cpu.increment(byte);

    mmu.write_byte(addr, new);

    12
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

    let new = cpu.decrement(byte);

    mmu.write_byte(addr, new);

    12
}

make_add_u8! {
    // Add value to the register A without considering carry.
    add_a_b, RegisterU8::B, false;
    add_a_c, RegisterU8::C, false;
    add_a_d, RegisterU8::D, false;
    add_a_e, RegisterU8::E, false;
    add_a_h, RegisterU8::H, false;
    add_a_l, RegisterU8::L, false;
    add_a_a, RegisterU8::A, false;
    // Add value to the register A considering carry.
    adc_a_b, RegisterU8::B, true;
    adc_a_c, RegisterU8::C, true;
    adc_a_d, RegisterU8::D, true;
    adc_a_e, RegisterU8::E, true;
    adc_a_h, RegisterU8::H, true;
    adc_a_l, RegisterU8::L, true;
    adc_a_a, RegisterU8::A, true;
}

pub fn add_a_hl(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let old = cpu.registers.read_u8(RegisterU8::A);
    let byte = mmu.read_byte(addr);

    let new = cpu.add(old, byte, false);

    cpu.registers.write_u8(RegisterU8::A, new);

    8
}

pub fn adc_a_hl(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let old = cpu.registers.read_u8(RegisterU8::A);
    let byte = mmu.read_byte(addr);

    let new = cpu.add(old, byte, true);

    cpu.registers.write_u8(RegisterU8::A, new);

    8
}

make_sub_u8! {
    // Subtract value from the register A without considering carry.
    sub_a_b, RegisterU8::B, false;
    sub_a_c, RegisterU8::C, false;
    sub_a_d, RegisterU8::D, false;
    sub_a_e, RegisterU8::E, false;
    sub_a_h, RegisterU8::H, false;
    sub_a_l, RegisterU8::L, false;
    sub_a_a, RegisterU8::A, false;
    // Subtract value from the register A considering carry.
    sbc_a_b, RegisterU8::B, true;
    sbc_a_c, RegisterU8::C, true;
    sbc_a_d, RegisterU8::D, true;
    sbc_a_e, RegisterU8::E, true;
    sbc_a_h, RegisterU8::H, true;
    sbc_a_l, RegisterU8::L, true;
    sbc_a_a, RegisterU8::A, true;
}

pub fn sub_a_hl(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let old = cpu.registers.read_u8(RegisterU8::A);
    let byte = mmu.read_byte(addr);

    let new = cpu.sub(old, byte, false);

    cpu.registers.write_u8(RegisterU8::A, new);

    8
}

pub fn sbc_a_hl(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let old = cpu.registers.read_u8(RegisterU8::A);
    let byte = mmu.read_byte(addr);

    let new = cpu.sub(old, byte, true);

    cpu.registers.write_u8(RegisterU8::A, new);

    8
}
