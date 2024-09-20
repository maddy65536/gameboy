use std::collections::HashSet;

#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; 0x10000], // just a flat array until i start the memory map stuff
    pub touched: HashSet<u16>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 0x10000],
            touched: HashSet::new(),
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        self.touched.insert(addr);
        self.ram[addr as usize] = val;
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        (self.read_u8(addr) as u16) | ((self.read_u8(addr + 1) as u16) << 8)
    }

    pub fn write_u16(&mut self, addr: u16, val: u16) {
        self.write_u8(addr, (val & 0x00FF) as u8);
        self.write_u8(addr + 1, ((val & 0xFF00) >> 8) as u8);
    }
}
