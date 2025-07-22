// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::cpu::Cpu;
use crate::registers::enums::Flag;

pub fn add(cpu: &mut Cpu, x: u8, y: u8) -> u8
{
    let new = x.wrapping_add(y);

    cpu.registers.set_flag(Flag::Z, new == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers
        .set_flag(Flag::H, (x & 0x0F) + (y & 0x0F) > 0xF);
    cpu.registers
        .set_flag(Flag::C, (x as u16) + (y as u16) > 0xFF);

    new
}

pub fn decrement(cpu: &mut Cpu, old: u8) -> u8
{
    let new = old.wrapping_sub(1);

    cpu.registers.set_flag(Flag::Z, new == 0);
    cpu.registers.set_flag(Flag::N, true);
    cpu.registers.set_flag(Flag::H, (old & 0x0F) == 0x00);

    new
}

pub fn increment(cpu: &mut Cpu, old: u8) -> u8
{
    let new = old.wrapping_add(1);

    cpu.registers.set_flag(Flag::Z, new == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, (old & 0x0F) == 0x0F);

    new
}
