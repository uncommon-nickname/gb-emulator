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

    pub fn from_slice(data: &[u8]) -> Self
    {
        Self::from_vec(data.to_vec())
    }

    pub fn from_vec(mut data: Vec<u8>) -> Self
    {
        assert!(data.len() <= Self::SIZE);

        data.resize_with(Self::SIZE, Default::default);

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

    fn write_byte(&mut self, addr: u16, val: u8)
    {
        let idx = self.addr_to_idx(addr);
        self.data[idx] = val;
    }
}
