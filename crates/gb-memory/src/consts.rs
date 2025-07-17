// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

pub mod rom
{
    pub const HEADER_SIZE: usize = 0x150;
    pub const TITLE_START: usize = 0x134;
    pub const TITLE_END: usize = 0x144;
    pub const CGB_FLAG: usize = 0x143;
    pub const CARTRIDGE_TYPE: usize = 0x147;
    pub const ROM_SIZE: usize = 0x148;
    pub const RAM_SIZE: usize = 0x149;
    pub const CHECKSUM: usize = 0x14D;
}
