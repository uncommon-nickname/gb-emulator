// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::MemoryAccess;
use crate::cartridge::header::Header;
use crate::chunk::MemoryChunk;

// NOTE: 17.07.2025
// This will not work for larger cartridges with alternate wiring.
// Very few games actually used it, so for now let's keep it simple.

#[derive(Debug)]
pub struct Mbc1
{
    ram_enabled: bool,
    // The banking mode can be in two states. Rom banking means
    // that 0x0000 - 0x3FFF address space points to the bank 0
    // of rom and bank 0 of ram. When set to ram banking, the
    // 0x0000 - 0x3FFF address space can be mapped to different
    // rom bank and the 0xA000 - 0xBFFF to a different ram bank.
    in_ram_banking_mode: bool,
    // 5-bit register keeping the rom bank number from the
    // 0x4000 - 0x7FFF region. The value it can hold can never
    // be zero. The range is [1, 31].
    rom_lower_rom_bits: u8,
    // If the 5 bits are not sufficient the secondary bank register
    // can be used providing additional two bits.
    ram_upper_rom_bits: u8,
    // First 16 KiB of the cartridge, which is always mapped
    // to the 0x0000 - 0x3FFF address space.
    rom_0x: MemoryChunk<0x0000, 0x3FFF>,
    // Rom banks which can be swapped, all of them are mapped
    // to the same address space and only once can be accessed
    // at the same time.
    rom: Vec<MemoryChunk<0x4000, 0x7FFF>>,
    // Ram banks which can be swapped, all of them are mapped
    // to the same address space.
    ram: Vec<MemoryChunk<0xA000, 0xBFFF>>,
}

impl Mbc1
{
    pub fn new(header: &Header, raw_rom: Vec<u8>) -> Self
    {
        const CHUNK: usize = MemoryChunk::<0x0000, 0x3FFF>::SIZE;

        let rom_0x = MemoryChunk::from_slice(&raw_rom[..CHUNK]);
        let ram = (0..header.ram_banks).map(|_| MemoryChunk::new()).collect();

        let rom: Vec<MemoryChunk<16384, 32767>> = raw_rom[CHUNK..]
            .chunks(CHUNK)
            .map(MemoryChunk::from_slice)
            .collect();

        assert_eq!(rom.len() + 1, header.rom_banks);

        Self {
            ram_enabled: false,
            in_ram_banking_mode: false,
            rom_lower_rom_bits: 0x01,
            ram_upper_rom_bits: 0x00,
            rom_0x,
            rom,
            ram,
        }
    }

    #[inline]
    fn current_rom_bank(&self) -> usize
    {
        let upper_bits = match self.in_ram_banking_mode {
            true => 0x00,
            false => self.ram_upper_rom_bits,
        };
        (self.rom_lower_rom_bits | (upper_bits << 5)) as usize
    }

    #[inline]
    fn current_ram_bank(&self) -> usize
    {
        match self.in_ram_banking_mode {
            true => self.ram_upper_rom_bits as usize,
            false => 0x00,
        }
    }
}

impl MemoryAccess for Mbc1
{
    fn read_byte(&self, addr: u16) -> u8
    {
        match addr {
            0x0000..=0x3FFF => self.rom_0x.read_byte(addr),
            0x4000..=0x7FFF => {
                let bank = self.current_rom_bank();
                self.rom[bank - 1].read_byte(addr)
            }
            0xA000..=0xBFFF if self.ram_enabled => {
                let bank = self.current_ram_bank();
                self.ram[bank].read_byte(addr)
            }
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8)
    {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enabled = (val & 0xF) == 0xA;
            }
            0x2000..=0x3FFF => {
                self.rom_lower_rom_bits = match val & 0x1F {
                    0 => 1,
                    val => val,
                };
            }
            0x4000..=0x5FFF => {
                self.ram_upper_rom_bits = val & 0x03;
            }
            0x6000..=0x7FFF => {
                self.in_ram_banking_mode = (val & 0x01) != 0;
            }
            0xA000..=0xBFFF if self.ram_enabled => {
                let bank = self.current_ram_bank();
                self.ram[bank].write_byte(addr, val);
            }
            _ => {}
        }
    }
}
