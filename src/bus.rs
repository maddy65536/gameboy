use crate::cart::Cart;

#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; 0x10000], // just a flat array until i start the memory map stuff
    cart: Cart,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Bus {
            ram: [0; 0x10000],
            cart,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.cart.read_u8(addr),
            0x4000..=0x7FFF => self.cart.read_u8(addr),
            0x8000..=0x9FFF => unimplemented!("tried to read vram"),
            0xA000..=0xBFFF => unimplemented!("tried to read cart ram"),
            0xC000..=0xCFFF => self.ram[addr as usize],
            0xD000..=0xDFFF => self.ram[addr as usize],
            0xE000..=0xFDFF => unimplemented!("tried to read echo ram"),
            0xFE00..=0xFE9F => unimplemented!("tried to read OAM"),
            0xFEA0..=0xFEFF => unimplemented!("tried to read FORBIDDEN MEMORY"),
            0xFF00..=0xFF7F => self.io_read_u8(addr),
            0xFF80..=0xFFFE => unimplemented!("tried to read HRAM"),
            0xFFFF => unimplemented!("tried to read IE"),
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x3FFF => unimplemented!("tried to write to cart rom bank 0"),
            0x4000..=0x7FFF => unimplemented!("tried to write to cart rom bank 01-NN"),
            0x8000..=0x9FFF => unimplemented!("tried to write to vram"),
            0xA000..=0xBFFF => unimplemented!("tried to write to cart ram"),
            0xC000..=0xCFFF => self.ram[addr as usize] = val,
            0xD000..=0xDFFF => self.ram[addr as usize] = val,
            0xE000..=0xFDFF => unimplemented!("tried to write to echo ram"),
            0xFE00..=0xFE9F => unimplemented!("tried to write to OAM"),
            0xFEA0..=0xFEFF => unimplemented!("tried to write to FORBIDDEN MEMORY"),
            0xFF00..=0xFF7F => self.io_write_u8(addr, val),
            0xFF80..=0xFFFE => unimplemented!("tried to write to HRAM"),
            0xFFFF => unimplemented!("tried to write to IE"),
        }
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        (self.read_u8(addr) as u16) | ((self.read_u8(addr + 1) as u16) << 8)
    }

    pub fn write_u16(&mut self, addr: u16, val: u16) {
        self.write_u8(addr, (val & 0x00FF) as u8);
        self.write_u8(addr + 1, ((val & 0xFF00) >> 8) as u8);
    }

    fn io_read_u8(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => unimplemented!("tried to read joypad input"),
            0xFF01..=0xFF02 => unimplemented!("tried to read serial transfer"),
            0xFF04..=0xFF07 => unimplemented!("tried to read timer and divider"),
            0xFF0F => unimplemented!("tried to read IF"),
            0xFF10..=0xFF26 => unimplemented!("tried to read audio register"),
            0xFF30..=0xFF3F => unimplemented!("tried to read wave pattern ram"),
            0xFF40..=0xFF4B => unimplemented!("tried to read LCD control"),
            0xFF4F => unimplemented!("tried to read CGB VRAM bank select"),
            0xFF50 => unimplemented!("tried to read boot rom flag"),
            0xFF51..=0xFF55 => unimplemented!("tried to read CGB VRAM DMA"),
            0xFF68..=0xFF6B => unimplemented!("tried to read CGB BG/OBJ palettes"),
            0xFF70 => unimplemented!("tried to read CGB WRAM bank select"),
            _ => unimplemented!("tried to read invalid i/o register {:#06X}????", addr),
        }
    }

    fn io_write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF00 => unimplemented!("tried to write to joypad input"),
            0xFF01..=0xFF02 => unimplemented!("tried to write to serial transfer"),
            0xFF04..=0xFF07 => unimplemented!("tried to write to timer and divider"),
            0xFF0F => unimplemented!("tried to write to IF"),
            0xFF10..=0xFF26 => unimplemented!("tried to write to audio register"),
            0xFF30..=0xFF3F => unimplemented!("tried to write to wave pattern ram"),
            0xFF40..=0xFF4B => unimplemented!("tried to write to LCD control"),
            0xFF4F => unimplemented!("tried to write to CGB VRAM bank select"),
            0xFF50 => unimplemented!("tried to write to boot rom flag"),
            0xFF51..=0xFF55 => unimplemented!("tried to write to CGB VRAM DMA"),
            0xFF68..=0xFF6B => unimplemented!("tried to write to CGB BG/OBJ palettes"),
            0xFF70 => unimplemented!("tried to write to CGB WRAM bank select"),
            _ => unimplemented!("tried to write to invalid i/o register {:#06X}????", addr),
        }
    }
}
