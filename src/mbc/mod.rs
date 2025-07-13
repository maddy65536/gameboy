use std::fmt::Debug;

mod mbc1;
mod mbc2;
mod mbc3;
mod no_mbc;

pub fn create_cart(rom: Vec<u8>, ram: Option<Vec<u8>>) -> Box<dyn Mbc> {
    // get cartridge header info
    let mbc = rom[0x0147];

    let mut cart = match mbc {
        0x00 => Box::new(no_mbc::NoMbc::new(rom)) as Box<dyn Mbc>,
        0x01 | 0x02 => Box::new(mbc1::Mbc1::new(rom, false)) as Box<dyn Mbc>,
        0x03 => Box::new(mbc1::Mbc1::new(rom, true)) as Box<dyn Mbc>,
        0x05 => Box::new(mbc2::Mbc2::new(rom, false)) as Box<dyn Mbc>,
        0x06 => Box::new(mbc2::Mbc2::new(rom, true)) as Box<dyn Mbc>,
        0x11 | 0x12 => Box::new(mbc3::Mbc3::new(rom, false)) as Box<dyn Mbc>,
        0x13 => Box::new(mbc3::Mbc3::new(rom, true)) as Box<dyn Mbc>,
        _ => panic!("unsuported MBC: {:#04x}", mbc),
    };
    if let Some(ram) = ram {
        cart.load_ram(ram);
    }
    cart
}

pub fn rom_size(rom: &[u8]) -> usize {
    32768 * ((rom[0x0148] as usize) << 1)
}

pub fn ram_size(rom: &[u8]) -> usize {
    match rom[0x0149] {
        0x00 => 0,
        0x01 => 2048, // questionable
        0x02 => 8192,
        0x03 => 32768,
        0x04 => 131072,
        0x05 => 65536,
        _ => panic!("unsupported rom bank size: {:#04x}", rom[0x0149]),
    }
}

pub trait Mbc
where
    Self: Debug,
{
    fn read_u8(&self, addr: u16) -> u8;
    fn write_u8(&mut self, addr: u16, val: u8);
    fn load_ram(&mut self, ram: Vec<u8>);
    fn dump_ram(&self) -> Vec<u8>;
    fn has_battery(&self) -> bool;
}
