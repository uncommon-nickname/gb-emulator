// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

mod bus;
mod chunk;
mod mmu;

pub use bus::MemoryBus;
pub use mmu::MMU;

pub trait MemoryAccess
{
    fn read_byte(&self, addr: u16) -> u8;
    fn read_word(&self, addr: u16) -> u16;
    fn write_byte(&mut self, addr: u16, val: u8);
    fn write_word(&mut self, addr: u16, val: u16);
}
