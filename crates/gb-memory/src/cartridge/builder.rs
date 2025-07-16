// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::MemoryAccess;
use crate::cartridge::errors::CartridgeError;
use crate::cartridge::header::{CartridgeType, Header};
use crate::cartridge::mbc::mbc0::MBC0;
use crate::cartridge::mbc::mbc1::MBC1;

pub struct Cartridge
{
    pub header: Header,
    mbc: Box<dyn MemoryAccess>,
}

// TODO: Detect which cartridge has a persistent `RAM` and save the state to the save file.
// TODO: Validate the checksum.
impl Cartridge
{
    pub fn from_file<P>(path: P) -> Result<Self, CartridgeError>
    where
        P: Into<PathBuf>,
    {
        let mut buf = Vec::new();
        File::open(path.into())?.read_to_end(&mut buf)?;

        let header = Header::new(&buf)?;

        let mbc: Box<dyn MemoryAccess> = match header.cartridge_type {
            CartridgeType::Mbc0 => Box::new(MBC0::new(buf)),
            _ => Box::new(MBC1::new(buf, header.ram_banks)),
        };

        Ok(Self { header, mbc })
    }
}

impl MemoryAccess for Cartridge
{
    fn read_byte(&self, addr: u16) -> u8
    {
        self.mbc.read_byte(addr)
    }

    fn write_byte(&mut self, addr: u16, val: u8)
    {
        self.mbc.write_byte(addr, val);
    }
}
