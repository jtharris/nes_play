// See:  https://bugzmanov.github.io/nes_ebook/chapter_4.html
const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const PRG_ROM: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

// These are to handle mirroring
fn ram_address(addr: u16) -> usize {
    (addr & 0x3FF) as usize
}

fn ppu_register_address(addr: u16) -> usize {
    (addr & 0x2007) as usize
}

fn rom_address(addr: u16) -> usize {
    (addr - PRG_ROM) as usize
}

pub struct Bus {
    cpu_vram: [u8; 0x0800],
    prg_rom: [u8; 0x8000]
}

impl Bus {
    pub fn empty() -> Self {
        Bus::new([0; 0x8000])
    }

    pub fn new(program: [u8; 0x8000]) -> Self {
       Bus {
           // Address space is 0x0000-0x2000 but it is mirrored twice due to only
           // allowing for 11 bits in the address bus.
           // See https://bugzmanov.github.io/nes_ebook/chapter_4.html
           cpu_vram: [0; 0x0800],
           prg_rom: program
       }
    }

    pub fn read_mem8(&self, addr: u16) -> u8 {
        match addr {
            RAM ..= RAM_MIRRORS_END => self.cpu_vram[ram_address(addr)],
            PPU_REGISTERS ..= PPU_REGISTERS_MIRRORS_END => todo!("PPU not supported yet!"),
            PRG_ROM ..= PRG_ROM_END => self.prg_rom[rom_address(addr)],
            _ => {
                // Todo:  something else here?
                println!("Ignoring memory read at:  {}", addr);
                0
            }
        }
    }

    pub fn write_mem8(&mut self, addr: u16, data: u8) {
        match addr {
            RAM ..= RAM_MIRRORS_END => self.cpu_vram[ram_address(addr)] = data,
            PPU_REGISTERS ..= PPU_REGISTERS_MIRRORS_END => todo!("PPU not supported yet!"),
            _ => {
                // Todo:  something else here?
                println!("Ignoring memory write at:  {}", addr);
            }
        }
    }

    pub fn read_mem16(&self, addr: u16) -> u16 {
        let bytes = [self.read_mem8(addr), self.read_mem8(addr+1)];
        u16::from_le_bytes(bytes)
    }

    pub fn write_mem16(&mut self, addr: u16, data: u16) {
        let bytes: [u8; 2] = data.to_le_bytes();
        self.write_mem8(addr, bytes[0]);
        self.write_mem8(addr+1, bytes[1]);
    }
}

#[cfg(test)]
mod test {
    use crate::bus::Bus;

    #[test]
    fn read_write_8bit_ram() {
        // Given
        let mut bus = Bus::empty();
        bus.write_mem8(0x0300, 0x1A);

        // Then
        assert_eq!(0x1A, bus.read_mem8(0x0300));
    }

    #[test]
    fn read_write_16bit_ram() {
        // Given
        let mut bus = Bus::empty();
        bus.write_mem16(0x0300, 0xFFE9);

        // Then
        assert_eq!(0xFFE9, bus.read_mem16(0x0300));
    }

    #[test]
    fn read_16bit_ram_little_endian() {
        // Given
        let mut bus = Bus::empty();
        bus.write_mem8(0x08A0, 0x10);
        bus.write_mem8(0x08A1, 0x28);

        // Then
        assert_eq!(0x2810, bus.read_mem16(0x08A0));
    }

    #[test]
    fn write_16bit_ram_little_endian() {
        // Given
        let mut bus = Bus::empty();

        // When
        bus.write_mem16(0x004A, 0xD82A);

        // Then
        assert_eq!(0x2A, bus.read_mem8(0x004A));
        assert_eq!(0xD8, bus.read_mem8(0x004B));
    }

}