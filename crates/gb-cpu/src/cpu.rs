// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::{MMU, MemoryAccess};

use crate::lookup_table::OPCODE_LOOKUP_TABLE;
use crate::registers::enums::{Flag, RegisterU16};
use crate::registers::wrapper::Registers;

#[derive(Debug, Default)]
pub struct Cpu
{
    pub registers: Registers,
}

impl Cpu
{
    pub fn new() -> Self
    {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn read_pc_byte(&mut self, mmu: &MMU<'_>) -> u8
    {
        let addr = self.registers.read_u16(RegisterU16::PC);
        let byte = mmu.read_byte(addr);

        let next = addr.wrapping_add(1);
        self.registers.write_u16(RegisterU16::PC, next);

        byte
    }

    pub fn read_pc_word(&mut self, mmu: &MMU<'_>) -> u16
    {
        let addr = self.registers.read_u16(RegisterU16::PC);
        let word = mmu.read_word(addr);

        let next = addr.wrapping_add(2);
        self.registers.write_u16(RegisterU16::PC, next);

        word
    }

    pub fn step(&mut self, mmu: MMU<'_>) -> u32
    {
        let opcode = self.read_pc_byte(&mmu);
        let instr_callable = OPCODE_LOOKUP_TABLE[opcode as usize];

        instr_callable(opcode, mmu, self)
    }
}

impl Cpu
{
    pub fn add(&mut self, x: u8, y: u8, consider_carry: bool) -> u8
    {
        let c = match consider_carry {
            true => self.registers.is_flag_set(Flag::C) as u8,
            false => 0,
        };

        let new = x.wrapping_add(y).wrapping_add(c);

        let half_carry_detected = (x & 0x0F) + (y & 0x0F) + c > 0xF;
        let carry_detected = (x as u16) + (y as u16) + (c as u16) > 0xFF;

        self.registers.set_flag(Flag::Z, new == 0);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::H, half_carry_detected);
        self.registers.set_flag(Flag::C, carry_detected);

        new
    }

    pub fn decrement(&mut self, x: u8) -> u8
    {
        let new = x.wrapping_sub(1);

        self.registers.set_flag(Flag::Z, new == 0);
        self.registers.set_flag(Flag::N, true);
        self.registers.set_flag(Flag::H, (x & 0x0F) == 0x00);

        new
    }

    pub fn increment(&mut self, x: u8) -> u8
    {
        let new = x.wrapping_add(1);

        self.registers.set_flag(Flag::Z, new == 0);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::H, (x & 0x0F) == 0x0F);

        new
    }

    pub fn sub(&mut self, x: u8, y: u8, consider_carry: bool) -> u8
    {
        let c = match consider_carry {
            true => self.registers.is_flag_set(Flag::C) as u8,
            false => 0,
        };

        let new = x.wrapping_sub(y).wrapping_sub(c);

        let half_carry_detected = (x & 0x0F) < ((y & 0x0F) + c);
        let carry_detected = (x as u16) < (y as u16 + c as u16);

        self.registers.set_flag(Flag::Z, new == 0);
        self.registers.set_flag(Flag::N, true);
        self.registers.set_flag(Flag::H, half_carry_detected);
        self.registers.set_flag(Flag::C, carry_detected);

        new
    }
}

impl Cpu
{
    pub fn and(&mut self, x: u8, y: u8) -> u8
    {
        let new = x & y;

        self.registers.set_flag(Flag::Z, new == 0);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::H, true);
        self.registers.set_flag(Flag::C, false);

        new
    }

    pub fn or(&mut self, x: u8, y: u8) -> u8
    {
        let new = x | y;

        self.registers.set_flag(Flag::Z, new == 0);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::C, false);

        new
    }

    pub fn xor(&mut self, x: u8, y: u8) -> u8
    {
        let new = x ^ y;

        self.registers.set_flag(Flag::Z, new == 0);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::C, false);

        new
    }

    pub fn cp(&mut self, x: u8, y: u8)
    {
        self.sub(x, y, false);
    }
}
