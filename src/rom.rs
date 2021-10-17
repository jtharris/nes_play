// Intended to support both iNes and iNes 2.0 formats (2.0 should be backwards compatible.
// Reference:  https://wiki.nesdev.org/w/index.php?title=INES#iNES_file_format
// https://wiki.nesdev.org/w/index.php?title=NES_2.0
use crate::rom::Mirroring::{Horizontal, Vertical};
use crate::rom::INesFormat::{INes2, INes, ArchaicINes};

use std::fmt;
use std::fmt::Formatter;

enum INesFormat {
    ArchaicINes,
    INes,
    INes2
}

enum Mirroring {
    Horizontal,
    Vertical
}

pub struct INes2Header {
    data: [u8; 16]
}

impl INes2Header {
    pub fn new(header_data: [u8; 16]) -> Self {
        INes2Header { data: header_data }
    }

    fn has_ines_identifier(&self) -> bool {
        &self.data[0..4] == [0x4E, 0x45, 0x53, 0x1A]
    }

    fn format(&self) -> INesFormat {
        if self.data[7] & 0x0C == 0x08 {
            // TODO:  Also check total size including byte 9 - need a reference to the whole rom?
            INes2
        } else if self.data[7] & 0x0C == 0 && &self.data[12..16] == [0, 0, 0 ,0] {
            INes
        } else {
            ArchaicINes
        }
    }

    fn prg_rom_size_bytes(&self) -> u16 {
        self.data[4] as u16 * 0x4000u16
    }

    fn chr_rom_size_bytes(&self) -> u16 {
        self.data[5] as u16 * 0x2000u16
    }

    fn mirroring(&self) -> Mirroring {
        if self.data[6] & 1 == 1 {
            Vertical
        } else {
            Horizontal
        }
    }
}

impl fmt::Display for INes2Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.has_ines_identifier() {
            let format = match self.format() {
                INes => "iNES",
                INes2 => "iNES 2.0",
                ArchaicINes => "Archaic iNES"
            };

            writeln!(f, "Format:  {}", format)?;
            writeln!(f, "PRG ROM size:  {} bytes", self.prg_rom_size_bytes())?;
            writeln!(f, "CHR ROM size:  {} bytes", self.chr_rom_size_bytes())
        } else {
            writeln!(f, "Invalid ROM")
        }
    }
}