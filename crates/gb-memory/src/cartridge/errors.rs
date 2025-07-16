// Copyright: (c) 2025, Wiktor Nowak
// GNU General Public License v3.0 (see LICENSE.md or https://www.gnu.org/licenses/gpl-3.0.txt)

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CartridgeError
{
    #[error("Problem with file system: {0}.")]
    FileSystem(#[from] std::io::Error),

    #[error("Problem with the rom: {0}.")]
    Rom(&'static str),
}
