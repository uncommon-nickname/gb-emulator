// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::MemoryAccess;
use crate::bus::MemoryBus;

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
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.bus.cartridge.read_byte(addr),
            0x8000..=0x9FFF => self.bus.vram.read_byte(addr),
            0xC000..=0xDFFF => self.bus.wram.read_byte(addr),
            _ => unimplemented!(),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8)
    {
        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.bus.cartridge.write_byte(addr, val),
            0x8000..=0x9FFF => self.bus.vram.write_byte(addr, val),
            0xC000..=0xDFFF => self.bus.wram.write_byte(addr, val),
            _ => unimplemented!(),
        }
    }
}
