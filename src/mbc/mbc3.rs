use crate::mbc::{Mbc, ram_size};

// no timer implementation

#[derive(Debug)]
pub struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    num_ram_banks: usize,
    ram_bank: usize,
    rom_bank: usize,
    ram_enable: u8,
    has_battery: bool,
}

impl Mbc3 {
    pub fn new(rom: Vec<u8>, has_battery: bool) -> Self {
        let ram_size = ram_size(&rom);
        Self {
            rom,
            ram: std::iter::repeat_n(0, ram_size).collect(),
            num_ram_banks: ram_size / 16384,
            ram_bank: 0,
            rom_bank: 0,
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
            _ => panic!("invalid read from MBC3 rom at address {addr:#06x}"),
        };
        self.rom[(bank << 14) | ((addr & 0x3FFF) as usize)]
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        if (self.ram_enable & 0x0F) != 0x0A {
            return 0xFF; // what does this actually return?
        }
        let bank = match self.num_ram_banks > 1 {
            false => 0,
            true => self.ram_bank,
        };
        self.ram[(bank << 13) | ((addr & 0x1FFF) as usize)]
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        if (self.ram_enable & 0x0F) != 0x0A {
            return;
        }
        let bank = match self.num_ram_banks > 1 {
            false => 0,
            true => self.ram_bank,
        };
        self.ram[(bank << 13) | ((addr & 0x1FFF) as usize)] = val;
    }
}

impl Mbc for Mbc3 {
    fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.read_rom(addr),
            0xA000..=0xBFFF => self.read_ram(addr),
            _ => panic!("invalid read from MBC3 at address {addr:#06x}"),
        }
    }

    fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enable = val,
            0x2000..=0x3FFF => self.rom_bank = (val & 0x7F) as usize,
            0x4000..=0x5FFF => self.ram_bank = (val & 0x07) as usize,
            0x6000..=0x7FFF => (),
            0xA000..=0xBFFF => self.write_ram(addr, val),
            _ => panic!("invalid write to MBC3 at address {addr:#06x}"),
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
