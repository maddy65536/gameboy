use crate::mbc::{Mbc, ram_size, rom_size};

#[derive(Debug)]
pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    num_rom_banks: usize,
    num_ram_banks: usize,
    ram_bank: usize,
    rom_bank: usize,
    ram_enable: u8,
    mode: bool,
    has_battery: bool,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>, has_battery: bool) -> Self {
        let ram_size = ram_size(&rom);
        let rom_size = rom_size(&rom);
        Self {
            rom,
            ram: std::iter::repeat_n(0, ram_size).collect(),
            num_rom_banks: rom_size / 16384,
            num_ram_banks: ram_size / 16384,
            ram_bank: 0,
            rom_bank: 0,
            ram_enable: 0,
            mode: false,
            has_battery,
        }
    }

    pub fn read_rom(&self, addr: u16) -> u8 {
        let bank = match addr {
            0x0000..=0x3FFF => match self.mode {
                false => 0,
                true => self.ram_bank << 5,
            },
            0x4000..=0x7FFF => {
                let mut bank = match self.rom_bank {
                    0 => 1,
                    n => n,
                };
                if self.num_rom_banks >= 64 {
                    bank |= self.ram_bank << 5;
                }
                bank
            }
            _ => panic!("invalid read from MBC1 rom at address {addr:#06x}"),
        };
        self.rom[(bank << 14) | ((addr & 0x3FFF) as usize)]
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        if (self.ram_enable & 0x0F) != 0x0A {
            return 0xFF; // what does this actually return?
        }
        let bank = match (self.num_ram_banks > 1, self.mode) {
            (false, _) => 0,
            (true, false) => 0,
            (true, true) => self.ram_bank,
        };
        self.ram[(bank << 13) | ((addr & 0x1FFF) as usize)]
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        if (self.ram_enable & 0x0F) != 0x0A {
            return;
        }
        let bank = match (self.num_ram_banks > 1, self.mode) {
            (false, _) => 0,
            (true, false) => 0,
            (true, true) => self.ram_bank,
        };
        self.ram[(bank << 13) | ((addr & 0x1FFF) as usize)] = val;
    }
}

impl Mbc for Mbc1 {
    fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.read_rom(addr),
            0xA000..=0xBFFF => self.read_ram(addr),
            _ => panic!("invalid read from MBC1 at address {addr:#06x}"),
        }
    }

    fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enable = val,
            0x2000..=0x3FFF => self.rom_bank = (val & 0x1F) as usize,
            0x4000..=0x5FFF => self.ram_bank = (val & 0x03) as usize,
            0x6000..=0x7FFF => self.mode = (val & 1) != 0,
            0xA000..=0xBFFF => self.write_ram(addr, val),
            _ => panic!("invalid write to MBC1 at address {addr:#06x}"),
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
