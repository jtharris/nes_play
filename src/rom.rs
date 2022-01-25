use std::convert::TryInto;
// Intended to support both iNes and iNes 2.0 formats (2.0 should be backwards compatible.
// Reference:  https://wiki.nesdev.org/w/index.php?title=INES#iNES_file_format
// https://wiki.nesdev.org/w/index.php?title=NES_2.0
use std::fmt;
use std::fmt::Formatter;
use crate::cpu::CPU;

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

pub struct INesRom {
    pub header: INes2Header,
    prg_rom: [u8; 0x4000]
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
            INesFormat::INes2
        } else if self.data[7] & 0x0C == 0 && &self.data[12..16] == [0, 0, 0 ,0] {
            INesFormat::INes
        } else {
            INesFormat::ArchaicINes
        }
    }

    // https://wiki.nesdev.org/w/index.php?title=NES_2.0#PRG-ROM_Area
    fn prg_rom_size_bytes(&self) -> usize {
        (self.data[4] as u16 * 0x4000u16) as usize
    }

    // https://wiki.nesdev.org/w/index.php?title=NES_2.0#CHR-ROM_Area
    fn chr_rom_size_bytes(&self) -> usize {
        (self.data[5] as u16 * 0x2000u16) as usize
    }

    fn has_trainer_data(&self) -> bool {
        self.data[6] & 0b0000_0100 == 0b0000_0100
    }

    fn mirroring(&self) -> Mirroring {
        if self.data[6] & 1 == 1 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        }
    }
}

impl fmt::Display for INes2Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.has_ines_identifier() {
            let format = match self.format() {
                INesFormat::INes => "iNES",
                INesFormat::INes2 => "iNES 2.0",
                INesFormat::ArchaicINes => "Archaic iNES"
            };

            writeln!(f, "Format:  {}", format)?;
            writeln!(f, "PRG ROM size:  {} bytes", self.prg_rom_size_bytes())?;
            writeln!(f, "CHR ROM size:  {} bytes", self.chr_rom_size_bytes())
        } else {
            writeln!(f, "Invalid ROM")
        }
    }
}

impl INesRom {
    pub fn new(contents: Vec<u8>) -> Self {
        let header_bytes = contents[0..16].try_into().expect("Header not found");
        let header = INes2Header::new(header_bytes);

        let trainer_data_size = match header.has_trainer_data() {
            true => 512,
            false => 0
        };

        let prg_rom_start = 16 + trainer_data_size;
        let prg_rom_end = prg_rom_start + header.prg_rom_size_bytes();

        let mut prg_rom: [u8; 0x4000] = [0x00; 0x4000];
        prg_rom.copy_from_slice(&contents[prg_rom_start..prg_rom_end]);

        INesRom{ header, prg_rom }
    }

    pub fn to_cpu(&self) -> CPU {
        CPU::new(self.prg_rom)
    }
}