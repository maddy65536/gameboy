use crate::mbc::Cart;
// just a plain cartridge with no mapper and no ram for now, i'll deal with those later
#[derive(Debug)]
pub struct NoMbc {
    rom: Vec<u8>,
}

impl NoMbc {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }
}

impl Cart for NoMbc {
    fn read_u8(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    // these don't do anything on an unbanked rom
    fn write_u8(&mut self, _addr: u16, _val: u8) {}

    fn load_ram(&mut self, _ram: Vec<u8>) {}

    fn dump_ram(&self) -> Option<Vec<u8>> {
        None
    }
}
