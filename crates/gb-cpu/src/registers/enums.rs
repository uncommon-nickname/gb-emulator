// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegisterU8
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegisterU16
{
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegisterGroup
{
    Standard,
    PushPop,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flag
{
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}

impl RegisterU8
{
    pub fn from_bits(bits: u8) -> Self
    {
        match bits {
            0b000 => RegisterU8::B,
            0b001 => RegisterU8::C,
            0b010 => RegisterU8::D,
            0b011 => RegisterU8::E,
            0b100 => RegisterU8::H,
            0b101 => RegisterU8::L,
            0b111 => RegisterU8::A,
            _ => unreachable!(),
        }
    }
}

impl RegisterU16
{
    pub fn from_bits(bits: u8, group: RegisterGroup) -> Self
    {
        match bits {
            0b00 => RegisterU16::BC,
            0b01 => RegisterU16::DE,
            0b10 => RegisterU16::HL,
            0b11 if group == RegisterGroup::Standard => RegisterU16::SP,
            0b11 if group == RegisterGroup::PushPop => RegisterU16::AF,
            _ => unreachable!(),
        }
    }
}
