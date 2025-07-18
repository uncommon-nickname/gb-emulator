// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use gb_memory::{MMU, MemoryAccess};

use crate::lookup_table::LOOKUP_TABLE;
use crate::registers::wrapper::Registers;

#[derive(Debug, Default)]
pub struct Cpu
{
    pub registers: Registers,
}

impl Cpu
{
    pub fn new() -> Self
    {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn read_pc_byte(&mut self, mmu: &MMU<'_>) -> u8
    {
        let addr = self.registers.read_pc();
        let byte = mmu.read_byte(addr);

        self.registers.increment_pc(1);

        byte
    }

    pub fn read_pc_word(&mut self, mmu: &MMU<'_>) -> u16
    {
        let addr = self.registers.read_pc();
        let word = mmu.read_word(addr);

        self.registers.increment_pc(2);

        word
    }

    pub fn step(&mut self, mut mmu: MMU<'_>) -> u32
    {
        let opcode = self.read_pc_byte(&mmu);
        let instr_callable = LOOKUP_TABLE[opcode as usize];

        instr_callable(opcode, &mut mmu, self)
    }
}
