// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::MMU;

use crate::cpu::Cpu;

pub fn nop(_opcode: u8, _mmu: MMU<'_>, _cpu: &mut Cpu) -> u32
{
    4
}
