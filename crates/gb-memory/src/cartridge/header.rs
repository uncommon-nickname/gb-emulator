// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::cartridge::errors::CartridgeError;

#[repr(u8)]
#[derive(Clone, Copy, FromPrimitive)]
pub enum CgbFlag
{
    SupportsEnhancements = 0x80,
    CgbOnly              = 0xC0,
}

#[repr(u8)]
#[derive(Clone, Copy, FromPrimitive)]
pub enum CartridgeType
{
    Mbc0                  = 0x00,
    Mbc1                  = 0x01,
    Mbc1WithRam           = 0x02,
    Mbc1WithRamAndBattery = 0x03,
}

pub struct Header
{
    pub title: String,
    pub cgb: CgbFlag,
    pub cartridge_type: CartridgeType,
    pub rom_banks: usize,
    pub ram_banks: usize,
    pub header_checksum: u8,
}

impl Header
{
    pub fn new(rom: &[u8]) -> Result<Self, CartridgeError>
    {
        if rom.len() < 0x150 {
            return Err(CartridgeError::Header(
                "Rom is too short to contain a header.",
            ));
        }
        let title = read_game_title(rom);

        let cgb = CgbFlag::from_u8(rom[0x143])
            .ok_or(CartridgeError::Header("Invalid value of cgb flag."))?;

        let cartridge_type = CartridgeType::from_u8(rom[0x147])
            .ok_or(CartridgeError::Header("Invalid value for catridge type."))?;

        let rom_banks = read_rom_size(rom);
        let ram_banks = read_ram_size(rom);

        Ok(Self {
            title,
            cgb,
            cartridge_type,
            rom_banks,
            ram_banks,
            header_checksum: rom[0x14D],
        })
    }
}

fn read_game_title(rom: &[u8]) -> String
{
    let title_bytes = &rom[0x134..0x144];
    String::from_utf8_lossy(title_bytes).to_string()
}

fn read_rom_size(rom: &[u8]) -> usize
{
    match rom[0x148] {
        0x00 => 2,
        0x01 => 4,
        0x02 => 8,
        0x03 => 16,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x08 => 512,
        _ => 2,
    }
}

fn read_ram_size(rom: &[u8]) -> usize
{
    match rom[0x149] {
        0x00 => 0,
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 64,
        _ => 0,
    }
}
