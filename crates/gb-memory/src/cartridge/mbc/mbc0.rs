// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::MemoryAccess;
use crate::chunk::MemoryChunk;

#[derive(Debug)]
pub struct Mbc0
{
    rom: MemoryChunk<0x0000, 0x7FFF>,
}

impl Mbc0
{
    pub fn new(rom: Vec<u8>) -> Self
    {
        let rom = MemoryChunk::from_vec(rom);
        Self { rom }
    }
}

impl MemoryAccess for Mbc0
{
    // `MBC0` allows read-only access to the `ROM` data.
    fn read_byte(&self, addr: u16) -> u8
    {
        self.rom.read_byte(addr)
    }

    // `MBC0` does not support any writing operations.
    fn write_byte(&mut self, _: u16, _: u8) {}
}
