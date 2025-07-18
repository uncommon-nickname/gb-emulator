// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::registers::enums::{Flag, RegisterU8, RegisterU16};

#[derive(Debug, Default)]
pub struct Registers
{
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    sp: u16,
    pc: u16,
}

impl Registers
{
    pub const fn new() -> Self
    {
        Self {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            f: 0x00,
            sp: 0x00,
            pc: 0x00,
        }
    }

    pub fn read_u8(&self, reg: RegisterU8) -> u8
    {
        match reg {
            RegisterU8::A => self.a,
            RegisterU8::B => self.b,
            RegisterU8::C => self.c,
            RegisterU8::D => self.d,
            RegisterU8::E => self.e,
            RegisterU8::H => self.h,
            RegisterU8::L => self.l,
        }
    }

    pub fn write_u8(&mut self, reg: RegisterU8, val: u8)
    {
        match reg {
            RegisterU8::A => self.a = val,
            RegisterU8::B => self.b = val,
            RegisterU8::C => self.c = val,
            RegisterU8::D => self.d = val,
            RegisterU8::E => self.e = val,
            RegisterU8::H => self.h = val,
            RegisterU8::L => self.l = val,
        }
    }

    pub fn read_u16(&self, reg: RegisterU16) -> u16
    {
        match reg {
            RegisterU16::AF => make_u16(self.a, self.f),
            RegisterU16::BC => make_u16(self.b, self.c),
            RegisterU16::DE => make_u16(self.d, self.e),
            RegisterU16::HL => make_u16(self.h, self.l),
            RegisterU16::SP => self.sp,
        }
    }

    pub fn write_u16(&mut self, reg: RegisterU16, val: u16)
    {
        match reg {
            RegisterU16::AF => {
                let (a, f) = make_u8(val);
                self.a = a;
                self.f = f & 0xF0; // Flag uses only 4 bits.
            }
            RegisterU16::BC => (self.b, self.c) = make_u8(val),
            RegisterU16::DE => (self.d, self.e) = make_u8(val),
            RegisterU16::HL => (self.h, self.l) = make_u8(val),
            RegisterU16::SP => self.sp = val,
        }
    }

    #[inline]
    pub fn read_pc(&self) -> u16
    {
        self.pc
    }

    #[inline]
    pub fn increment_pc(&mut self, by: u16)
    {
        self.pc = self.pc.wrapping_add(by);
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool)
    {
        let bits = flag as u8;

        if val {
            self.f |= bits;
        } else {
            self.f &= bits;
        }
        self.f &= 0xF0;
    }

    pub fn increment_u8(&mut self, reg: RegisterU8)
    {
        let old = self.read_u8(reg);
        let new = old.wrapping_add(1);

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, (old & 0x0F) == 0x0F);

        self.write_u8(reg, new);
    }

    pub fn decrement_u8(&mut self, reg: RegisterU8)
    {
        let old = self.read_u8(reg);
        let new = old.wrapping_sub(1);

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, (old & 0x0F) == 0x00);

        self.write_u8(reg, new);
    }
}

#[inline]
fn make_u16(high: u8, low: u8) -> u16
{
    ((high as u16) << 8) | low as u16
}

#[inline]
fn make_u8(val: u16) -> (u8, u8)
{
    let high = ((val >> 8) & 0xFF) as u8;
    let low = (val & 0xFF) as u8;
    (high, low)
}
