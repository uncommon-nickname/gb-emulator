// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::MemoryAccess;
use crate::chunk::MemoryChunk;

#[derive(Debug)]
pub struct MBC1
{
    banking_mode: u8,

    rom_bank: u8,
    static_rom: MemoryChunk<0x0000, 0x3FFF>,
    swap_rom: Vec<MemoryChunk<0x4000, 0x7FFF>>,

    ram_enabled: bool,
    ram_bank: u8,
    swap_ram: Vec<MemoryChunk<0xA000, 0xBFFF>>,
}

impl MBC1
{
    pub fn new(rom: Vec<u8>, ram_banks: usize) -> Self
    {
        const SIZE: usize = MemoryChunk::<0x0000, 0x3FFF>::SIZE;

        let static_rom = MemoryChunk::from_slice(&rom[0..SIZE]);

        let swap_rom = rom[SIZE..]
            .chunks(SIZE)
            .map(MemoryChunk::from_slice)
            .collect();

        let swap_ram = (0..ram_banks).map(|_| MemoryChunk::new()).collect();

        Self {
            banking_mode: 0,
            rom_bank: 1,
            static_rom,
            swap_rom,
            ram_enabled: false,
            ram_bank: 0,
            swap_ram,
        }
    }
}

impl MBC1
{
    fn read_rom_x0(&self, addr: u16) -> u8
    {
        // In banking mode `0` we simply read from the static `ROM` data.
        if self.banking_mode == 0 {
            return self.static_rom.read_byte(addr);
        }
        // In banking mode `1` with extended `ROM` banking, we can have
        // some data mapped here, specified by three bits in the `rom_bank`.
        let bank = (self.rom_bank & 0xE0) as usize;

        if bank == 0 {
            self.static_rom.read_byte(addr)
        } else {
            self.swap_rom[bank - 1].read_byte(addr & 0x3FFF)
        }
    }

    fn rom_banks(&self) -> u8
    {
        (self.swap_rom.len() + 1) as u8
    }
}

impl MemoryAccess for MBC1
{
    fn read_byte(&self, addr: u16) -> u8
    {
        match addr {
            0x0000..=0x3FFF => self.read_rom_x0(addr),
            0x4000..=0x7FFF => {
                let bank = (self.rom_bank - 1) as usize;
                self.swap_rom[bank].read_byte(addr)
            }
            0xA000..=0xBFFF if self.ram_enabled => {
                let bank = match self.banking_mode {
                    1 => self.ram_bank as usize,
                    _ => 0,
                };
                self.swap_ram[bank].read_byte(addr)
            }
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8)
    {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enabled = val & 0xF == 0xA;
            }
            0x2000..=0x3FFF => {
                let mut lower = val & 0x1F;

                if lower == 0 {
                    lower = 1;
                }
                self.rom_bank = ((self.rom_bank & 0x60) | lower) % self.rom_banks();
            }
            0x4000..=0x5FFF => {
                let upper = (val & 0x03) & (self.rom_banks() >> 5);
                self.rom_bank = self.rom_bank & 0x1F | (upper << 5);
                self.ram_bank = val & 0x03;
            }
            0x6000..=0x7FFF => {
                self.banking_mode = val & 0x01;
            }
            0xA000..=0xBFFF if self.ram_enabled => {
                let bank = match self.banking_mode {
                    1 => self.ram_bank as usize,
                    _ => 0,
                };
                self.swap_ram[bank].write_byte(addr, val);
            }
            _ => {}
        }
    }
}
