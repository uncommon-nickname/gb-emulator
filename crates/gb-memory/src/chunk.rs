// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::MemoryAccess;

#[derive(Debug, Default)]
pub struct MemoryChunk<const START_ADDR: u16, const END_ADDR: u16>
{
    data: Vec<u8>,
}

impl<const START_ADDR: u16, const END_ADDR: u16> MemoryChunk<START_ADDR, END_ADDR>
{
    pub const SIZE: usize = (END_ADDR - START_ADDR) as usize;

    pub fn new() -> Self
    {
        let data = vec![0x0; Self::SIZE];
        Self { data }
    }

    #[inline]
    fn addr_to_idx(&self, addr: u16) -> usize
    {
        (addr - START_ADDR) as usize
    }
}

impl<const START_ADDR: u16, const END_ADDR: u16> MemoryAccess for MemoryChunk<START_ADDR, END_ADDR>
{
    fn read_byte(&self, addr: u16) -> u8
    {
        let idx = self.addr_to_idx(addr);
        self.data[idx]
    }

    fn read_word(&self, addr: u16) -> u16
    {
        let low = self.read_byte(addr) as u16;
        let high = self.read_byte(addr + 1) as u16;

        low | (high << 8)
    }

    fn write_byte(&mut self, addr: u16, val: u8)
    {
        let idx = self.addr_to_idx(addr);
        self.data[idx] = val;
    }

    fn write_word(&mut self, addr: u16, val: u16)
    {
        let low = (val & 0xFF) as u8;
        let high = (val >> 8) as u8;

        self.write_byte(addr, low);
        self.write_byte(addr + 1, high);
    }
}
