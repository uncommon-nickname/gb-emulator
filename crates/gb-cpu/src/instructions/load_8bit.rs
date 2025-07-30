// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::{MMU, MemoryAccess};

use crate::cpu::Cpu;
use crate::registers::enums::{RegisterU8, RegisterU16};

macro_rules! make_ld_n8
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let val = cpu.read_pc_byte(&mmu);
                cpu.registers.write_u8($reg, val);
                8
            }
        )*
    };
}

macro_rules! make_ld_u8_u8
{
    ($($name:ident, $to: expr, $from: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, _mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let val = cpu.registers.read_u8($from);
                cpu.registers.write_u8($to, val);
                4
            }
        )*
    };
}

macro_rules! make_ld_u8_hl
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let addr = cpu.registers.read_u16(RegisterU16::HL);
                let val = mmu.read_byte(addr);

                cpu.registers.write_u8($reg, val);

                8
            }
        )*
    };
}

macro_rules! make_ld_hl_u8
{
    ($($name:ident, $reg: expr);* $(;)?) => {
        $(
            pub fn $name(_opcode: u8, mut mmu: MMU<'_>, cpu: &mut Cpu) -> u32
            {
                let addr = cpu.registers.read_u16(RegisterU16::HL);
                let val = cpu.registers.read_u8($reg);

                mmu.write_byte(addr, val);

                8
            }
        )*
    };
}

pub fn ld_bc_a(_opcode: u8, mut mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::BC);
    let val = cpu.registers.read_u8(RegisterU8::A);

    mmu.write_byte(addr, val);

    8
}

pub fn ld_de_a(_opcode: u8, mut mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::DE);
    let val = cpu.registers.read_u8(RegisterU8::A);

    mmu.write_byte(addr, val);

    8
}

pub fn ldi_hl_a(_opcode: u8, mut mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let val = cpu.registers.read_u8(RegisterU8::A);

    cpu.registers.write_u16(RegisterU16::HL, addr + 1);
    mmu.write_byte(addr, val);

    8
}

pub fn ldd_hl_a(_opcode: u8, mut mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let val = cpu.registers.read_u8(RegisterU8::A);

    cpu.registers.write_u16(RegisterU16::HL, addr - 1);
    mmu.write_byte(addr, val);

    8
}

pub fn ld_a_bc(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::BC);
    let val = mmu.read_byte(addr);

    cpu.registers.write_u8(RegisterU8::A, val);

    8
}

pub fn ld_a_de(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::DE);
    let val = mmu.read_byte(addr);

    cpu.registers.write_u8(RegisterU8::A, val);

    8
}

pub fn ldi_a_hl(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let val = mmu.read_byte(addr);

    cpu.registers.write_u16(RegisterU16::HL, addr + 1);
    cpu.registers.write_u8(RegisterU8::A, val);

    8
}

pub fn ldd_a_hl(_opcode: u8, mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let addr = cpu.registers.read_u16(RegisterU16::HL);
    let val = mmu.read_byte(addr);

    cpu.registers.write_u16(RegisterU16::HL, addr - 1);
    cpu.registers.write_u8(RegisterU8::A, val);

    8
}

make_ld_n8! {
    ld_b_n8, RegisterU8::B;
    ld_d_n8, RegisterU8::D;
    ld_h_n8, RegisterU8::H;
    ld_c_n8, RegisterU8::C;
    ld_e_n8, RegisterU8::E;
    ld_l_n8, RegisterU8::L;
    ld_a_n8, RegisterU8::A;
}

pub fn ld_hl_n8(_opcode: u8, mut mmu: MMU<'_>, cpu: &mut Cpu) -> u32
{
    let val = cpu.read_pc_byte(&mmu);
    let addr = cpu.registers.read_u16(RegisterU16::HL);

    mmu.write_byte(addr, val);

    12
}

make_ld_u8_u8! {
    // Load value from other register to register B.
    ld_b_b, RegisterU8::B, RegisterU8::B;
    ld_b_c, RegisterU8::B, RegisterU8::C;
    ld_b_d, RegisterU8::B, RegisterU8::D;
    ld_b_e, RegisterU8::B, RegisterU8::E;
    ld_b_h, RegisterU8::B, RegisterU8::H;
    ld_b_l, RegisterU8::B, RegisterU8::L;
    ld_b_a, RegisterU8::B, RegisterU8::A;
    // Load value from other register to register C.
    ld_c_b, RegisterU8::C, RegisterU8::B;
    ld_c_c, RegisterU8::C, RegisterU8::C;
    ld_c_d, RegisterU8::C, RegisterU8::D;
    ld_c_e, RegisterU8::C, RegisterU8::E;
    ld_c_h, RegisterU8::C, RegisterU8::H;
    ld_c_l, RegisterU8::C, RegisterU8::L;
    ld_c_a, RegisterU8::C, RegisterU8::A;
    // Load value from other register to register D.
    ld_d_b, RegisterU8::D, RegisterU8::B;
    ld_d_c, RegisterU8::D, RegisterU8::C;
    ld_d_d, RegisterU8::D, RegisterU8::D;
    ld_d_e, RegisterU8::D, RegisterU8::E;
    ld_d_h, RegisterU8::D, RegisterU8::H;
    ld_d_l, RegisterU8::D, RegisterU8::L;
    ld_d_a, RegisterU8::D, RegisterU8::A;
    // Load value from other register to register E.
    ld_e_b, RegisterU8::E, RegisterU8::B;
    ld_e_c, RegisterU8::E, RegisterU8::C;
    ld_e_d, RegisterU8::E, RegisterU8::D;
    ld_e_e, RegisterU8::E, RegisterU8::E;
    ld_e_h, RegisterU8::E, RegisterU8::H;
    ld_e_l, RegisterU8::E, RegisterU8::L;
    ld_e_a, RegisterU8::E, RegisterU8::A;
    // Load value from other register to register H.
    ld_h_b, RegisterU8::H, RegisterU8::B;
    ld_h_c, RegisterU8::H, RegisterU8::C;
    ld_h_d, RegisterU8::H, RegisterU8::D;
    ld_h_e, RegisterU8::H, RegisterU8::E;
    ld_h_h, RegisterU8::H, RegisterU8::H;
    ld_h_l, RegisterU8::H, RegisterU8::L;
    ld_h_a, RegisterU8::H, RegisterU8::A;
    // Load value from other register to register L.
    ld_l_b, RegisterU8::L, RegisterU8::B;
    ld_l_c, RegisterU8::L, RegisterU8::C;
    ld_l_d, RegisterU8::L, RegisterU8::D;
    ld_l_e, RegisterU8::L, RegisterU8::E;
    ld_l_h, RegisterU8::L, RegisterU8::H;
    ld_l_l, RegisterU8::L, RegisterU8::L;
    ld_l_a, RegisterU8::L, RegisterU8::A;
    // Load value from other register to register A.
    ld_a_b, RegisterU8::A, RegisterU8::B;
    ld_a_c, RegisterU8::A, RegisterU8::C;
    ld_a_d, RegisterU8::A, RegisterU8::D;
    ld_a_e, RegisterU8::A, RegisterU8::E;
    ld_a_h, RegisterU8::A, RegisterU8::H;
    ld_a_l, RegisterU8::A, RegisterU8::L;
    ld_a_a, RegisterU8::A, RegisterU8::A;
}

make_ld_u8_hl! {
    ld_b_hl, RegisterU8::B;
    ld_c_hl, RegisterU8::C;
    ld_d_hl, RegisterU8::D;
    ld_e_hl, RegisterU8::E;
    ld_h_hl, RegisterU8::H;
    ld_l_hl, RegisterU8::L;
    ld_a_hl, RegisterU8::A;
}

make_ld_hl_u8! {
    ld_hl_b, RegisterU8::B;
    ld_hl_c, RegisterU8::C;
    ld_hl_d, RegisterU8::D;
    ld_hl_e, RegisterU8::E;
    ld_hl_h, RegisterU8::H;
    ld_hl_l, RegisterU8::L;
    ld_hl_a, RegisterU8::A;
}
