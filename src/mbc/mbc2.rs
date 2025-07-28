use crate::mbc::Mbc;

#[derive(Debug)]
pub struct Mbc2 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_enable: u8,
    has_battery: bool,
}

impl Mbc2 {
    pub fn new(rom: Vec<u8>, has_battery: bool) -> Self {
        Self {
            rom,
            ram: std::iter::repeat_n(0, 512).collect(),
            rom_bank: 1,
            ram_enable: 0,
            has_battery,
        }
    }

    pub fn read_rom(&self, addr: u16) -> u8 {
        let bank = match addr {
            0x0000..=0x3FFF => 0,
            0x4000..=0x7FFF => match self.rom_bank {
                0 => 1,
                n => n,
            },
            _ => panic!("invalid read from MBC2 rom at address {addr:#06x}"),
        };
        self.rom[((bank as usize) << 14) | ((addr & 0x3FFF) as usize)]
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        if (self.ram_enable & 0x0F) != 0x0A {
            return 0xFF; // what does this actually return?
        }
        self.ram[(addr & 0x01FF) as usize]
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        if (self.ram_enable & 0x0F) != 0x0A {
            //println!("whaaaa {}", self.ram_enable);
            return;
        }
        // only write the bottom 4 bits because MBC2 uses half byte ram for some reason
        self.ram[(addr & 0x01FF) as usize] = val & 0x0F;
    }
}

impl Mbc for Mbc2 {
    fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.read_rom(addr),
            0xA000..=0xBFFF => self.read_ram(addr),
            _ => panic!("invalid read from MBC2 at address {addr:#06x}"),
        }
    }

    fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x3FFF => {
                if (addr & 0x100) == 0 {
                    self.ram_enable = val;
                } else {
                    self.rom_bank = (val & 0x0F) as usize;
                }
            }
            0xA000..=0xBFFF => self.write_ram(addr, val),
            _ => panic!("invalid write to MBC2 at address {addr:#06x}"),
        }
    }

    fn load_ram(&mut self, ram: Vec<u8>) {
        self.ram = ram
    }

    fn dump_ram(&self) -> Vec<u8> {
        self.ram.clone()
    }

    fn has_battery(&self) -> bool {
        self.has_battery
    }
}
