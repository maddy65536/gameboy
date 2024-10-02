// just a plain cartridge with no mapper and no ram for now, i'll deal with those later
#[derive(Debug)]
pub struct Cart {
    rom: Vec<u8>,
}

impl Cart {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    pub fn write_u8(&mut self, addr: u16) {
        unimplemented!("tried to write to cart")
    }
}
