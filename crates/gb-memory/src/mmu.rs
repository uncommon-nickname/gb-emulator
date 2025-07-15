// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::MemoryAccess;
use crate::bus::MemoryBus;

#[derive(Debug)]
pub struct MMU<'a>
{
    bus: &'a mut MemoryBus,
}

impl<'a> MMU<'a>
{
    pub fn new(bus: &'a mut MemoryBus) -> Self
    {
        Self { bus }
    }
}

impl<'a> MemoryAccess for MMU<'a>
{
    fn read_byte(&self, addr: u16) -> u8
    {
        match addr {
            0x8000..=0x9FFF => self.bus.vram.read_byte(addr),
            0xC000..=0xDFFF => self.bus.wram.read_byte(addr),
            _ => unimplemented!(),
        }
    }

    fn read_word(&self, addr: u16) -> u16
    {
        match addr {
            0x8000..=0x9FFF => self.bus.vram.read_word(addr),
            0xC000..=0xDFFF => self.bus.wram.read_word(addr),
            _ => unimplemented!(),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8)
    {
        match addr {
            0x8000..=0x9FFF => self.bus.vram.write_byte(addr, val),
            0xC000..=0xDFFF => self.bus.wram.write_byte(addr, val),
            _ => unimplemented!(),
        }
    }

    fn write_word(&mut self, addr: u16, val: u16)
    {
        match addr {
            0x8000..=0x9FFF => self.bus.vram.write_word(addr, val),
            0xC000..=0xDFFF => self.bus.wram.write_word(addr, val),
            _ => unimplemented!(),
        }
    }
}
