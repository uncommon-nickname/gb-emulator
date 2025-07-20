// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::{MMU, MemoryAccess};

use crate::cpu::Cpu;
use crate::registers::enums::{RegisterU8, RegisterU16};

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

macro_rules! make_ld_n8 {
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
