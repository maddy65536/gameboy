use std::fmt::Debug;

pub mod no_mbc;

pub fn create_cart(rom: Vec<u8>, ram: Option<Vec<u8>>) -> Box<dyn Cart> {
    let mbc = rom[0x0147];
    let mut cart = Box::new(match mbc {
        0x00 => no_mbc::NoMbc::new(rom),
        _ => panic!("unsuported MBC: {:#04x}", mbc),
    });
    if let Some(ram) = ram {
        cart.load_ram(ram);
    }
    cart
}

pub trait Cart
where
    Self: Debug,
{
    fn read_u8(&self, addr: u16) -> u8;
    fn write_u8(&mut self, addr: u16, val: u8);
    fn load_ram(&mut self, ram: Vec<u8>);
    fn dump_ram(&self) -> Option<Vec<u8>>;
}
