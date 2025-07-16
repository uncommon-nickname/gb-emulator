// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use crate::cartridge::builder::Cartridge;
use crate::chunk::MemoryChunk;
use crate::mmu::MMU;

pub struct MemoryBus
{
    pub cartridge: Cartridge,
    pub vram: MemoryChunk<0x8000, 0x9FFF>,
    pub wram: MemoryChunk<0xC000, 0xDFFF>,
}

impl MemoryBus
{
    pub fn new(cartridge: Cartridge) -> Self
    {
        Self {
            cartridge,
            vram: MemoryChunk::new(),
            wram: MemoryChunk::new(),
        }
    }

    pub fn mmu(&mut self) -> MMU<'_>
    {
        MMU::new(self)
    }
}
