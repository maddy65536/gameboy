use crate::joypad::Joypad;
use crate::mbc::Cart;
use crate::ppu::Ppu;
use crate::serial::Serial;
use crate::timer::Timer;

#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; 0x10000], // just a flat array until i start the memory map stuff
    cart: Box<dyn Cart>,
    pub timer: Timer,
    pub ppu: Ppu,
    pub joypad: Joypad,
    serial: Serial,
}

impl Bus {
    pub fn new(cart: Box<dyn Cart>) -> Self {
        Bus {
            ram: [0; 0x10000],
            cart,
            timer: Timer::new(),
            ppu: Ppu::new(),
            joypad: Joypad::new(),
            serial: Serial::new(),
        }
    }

    pub fn tick(&mut self, cycles: u64) {
        self.joypad.tick();
        self.ram[0xFF0F] |= (self.joypad.joypad_int as u8) << 4;
        self.joypad.joypad_int = false;

        self.serial.tick(cycles);
        self.ram[0xFF0F] |= (self.serial.serial_int as u8) << 3;
        self.serial.serial_int = false;

        self.timer.tick(cycles);
        self.ram[0xFF0F] |= (self.timer.timer_int as u8) << 2;
        self.timer.timer_int = false;

        self.ppu.tick(cycles);
        self.ram[0xFF0F] |= (self.ppu.stat_int as u8) << 1;
        self.ppu.stat_int = false;
        self.ram[0xFF0F] |= self.ppu.vblank_int as u8;
        self.ppu.vblank_int = false;
    }

    fn ram_read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn ram_write(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }

    // pub fn read_u8(&self, addr: u16) -> u8 {
    //     match addr {
    //         0x0000..=0x3FFF => self.cart.read_u8(addr),
    //         0x4000..=0x7FFF => self.cart.read_u8(addr),
    //         0x8000..=0x9FFF => unimplemented!("tried to read vram"),
    //         0xA000..=0xBFFF => unimplemented!("tried to read cart ram"),
    //         0xC000..=0xCFFF => self.ram_read(addr),
    //         0xD000..=0xDFFF => self.ram_read(addr),
    //         0xE000..=0xFDFF => unimplemented!("tried to read echo ram"),
    //         0xFE00..=0xFE9F => unimplemented!("tried to read OAM"),
    //         0xFEA0..=0xFEFF => unimplemented!("tried to read FORBIDDEN MEMORY"),
    //         0xFF00..=0xFF7F => self.io_read_u8(addr),
    //         0xFF80..=0xFFFE => self.ram_read(addr), // HRAM
    //         0xFFFF => self.ram_read(addr),          // IE
    //     }
    // }

    // pub fn write_u8(&mut self, addr: u16, val: u8) {
    //     match addr {
    //         0x0000..=0x3FFF => unimplemented!("tried to write to cart rom bank 0"),
    //         0x4000..=0x7FFF => unimplemented!("tried to write to cart rom bank 01-NN"),
    //         0x8000..=0x9FFF => unimplemented!("tried to write to vram"),
    //         0xA000..=0xBFFF => unimplemented!("tried to write to cart ram"),
    //         0xC000..=0xCFFF => self.ram_write(addr, val),
    //         0xD000..=0xDFFF => self.ram_write(addr, val),
    //         0xE000..=0xFDFF => unimplemented!("tried to write to echo ram"),
    //         0xFE00..=0xFE9F => unimplemented!("tried to write to OAM"),
    //         0xFEA0..=0xFEFF => unimplemented!("tried to write to FORBIDDEN MEMORY"),
    //         0xFF00..=0xFF7F => self.io_write_u8(addr, val),
    //         0xFF80..=0xFFFE => self.ram_write(addr, val), // HRAM
    //         0xFFFF => self.ram_write(addr, val),          // IE
    //     }
    // }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.cart.read_u8(addr), // cart rom bank 0
            0x4000..=0x7FFF => self.cart.read_u8(addr), // cart rom bank 01-NN
            0x8000..=0x9FFF => self.ppu.read_u8(addr),  // VRAM
            0xC000..=0xCFFF => self.ram_read(addr),
            0xD000..=0xDFFF => self.ram_read(addr),
            0xFE00..=0xFE9F => self.ppu.read_u8(addr), // OAM
            0xFF00..=0xFF7F => self.io_read_u8(addr),  // IO
            0xFF80..=0xFFFE => self.ram_read(addr),    // HRAM
            0xFFFF => self.ram_read(addr),             // IE
            _ => self.ram_read(addr),
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9FFF => self.ppu.write_u8(addr, val), // VRAM
            0xC000..=0xCFFF => self.ram_write(addr, val),
            0xD000..=0xDFFF => self.ram_write(addr, val),
            0xFE00..=0xFE9F => self.ppu.write_u8(addr, val), // OAM
            0xFF00..=0xFF7F => self.io_write_u8(addr, val),  // IO
            0xFF80..=0xFFFE => self.ram_write(addr, val),    // HRAM
            0xFFFF => self.ram_write(addr, val),             // IE
            _ => self.ram_write(addr, val),
        }
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        (self.read_u8(addr) as u16) | ((self.read_u8(addr + 1) as u16) << 8)
    }

    pub fn write_u16(&mut self, addr: u16, val: u16) {
        self.write_u8(addr, (val & 0x00FF) as u8);
        self.write_u8(addr + 1, ((val & 0xFF00) >> 8) as u8);
    }

    // fn io_read_u8(&self, addr: u16) -> u8 {
    //     match addr {
    //         0xFF00 => unimplemented!("tried to read joypad input"),
    //         0xFF01 => self.ram_read(addr), // serial data
    //         0xFF02 => unimplemented!("tried to read serial transfer"),
    //         0xFF04..=0xFF07 => self.timer.read_u8(addr),
    //         0xFF0F => self.ram_read(addr), // IF
    //         0xFF10..=0xFF26 => unimplemented!("tried to read audio register"),
    //         0xFF30..=0xFF3F => unimplemented!("tried to read wave pattern ram"),
    //         0xFF40..=0xFF4B => unimplemented!("tried to read LCD control"),
    //         0xFF4F => unimplemented!("tried to read CGB VRAM bank select"),
    //         0xFF50 => unimplemented!("tried to read boot rom flag"),
    //         0xFF51..=0xFF55 => unimplemented!("tried to read CGB VRAM DMA"),
    //         0xFF68..=0xFF6B => unimplemented!("tried to read CGB BG/OBJ palettes"),
    //         0xFF70 => unimplemented!("tried to read CGB WRAM bank select"),
    //         _ => unimplemented!("tried to read invalid i/o register {:#06X}????", addr),
    //     }
    // }

    // fn io_write_u8(&mut self, addr: u16, val: u8) {
    //     match addr {
    //         0xFF00 => unimplemented!("tried to write to joypad input"),
    //         0xFF01 => self.ram_write(addr, val), // serial data
    //         0xFF02 => {
    //             if val == 0x81 {
    //                 print!("{}", self.ram[0xFF01] as char)
    //             } else {
    //                 unimplemented!("tried something weird with serial, wrote {:#04X}", val)
    //             }
    //         }
    //         0xFF04..=0xFF07 => self.timer.write_u8(addr, val),
    //         0xFF0F => self.ram_write(addr, val), // IF
    //         0xFF10..=0xFF26 => unimplemented!("tried to write to audio register"),
    //         0xFF30..=0xFF3F => unimplemented!("tried to write to wave pattern ram"),
    //         0xFF40..=0xFF4B => unimplemented!("tried to write to LCD control"),
    //         0xFF4F => unimplemented!("tried to write to CGB VRAM bank select"),
    //         0xFF50 => unimplemented!("tried to write to boot rom flag"),
    //         0xFF51..=0xFF55 => unimplemented!("tried to write to CGB VRAM DMA"),
    //         0xFF68..=0xFF6B => unimplemented!("tried to write to CGB BG/OBJ palettes"),
    //         0xFF70 => unimplemented!("tried to write to CGB WRAM bank select"),
    //         _ => unimplemented!("tried to write to invalid i/o register {:#06X}????", addr),
    //     }
    // }

    fn io_read_u8(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.joypad.read_u8(),              // joypad
            0xFF01 | 0xFF02 => self.serial.read_u8(addr), // serial data
            0xFF04..=0xFF07 => self.timer.read_u8(addr),
            0xFF46 => 0, // what does reading from the dma register do?
            0xFF40..0xFF46 | 0xFF47..=0xFF4B => self.ppu.read_u8(addr),
            0xFF0F => self.ram_read(addr), // IF
            _ => 0,
        }
    }

    fn io_write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF00 => self.joypad.write_u8(val),                // joypad
            0xFF01 | 0xFF02 => self.serial.write_u8(addr, val), // serial data
            0xFF04..=0xFF07 => self.timer.write_u8(addr, val),
            0xFF46 => self.do_dma(val), // dma lives in the bus to make things easier
            0xFF40..0xFF46 | 0xFF47..=0xFF4B => self.ppu.write_u8(addr, val),
            0xFF0F => self.ram_write(addr, val), // IF
            _ => (),
        }
    }

    // not that accurate but i'm just gonna trust roms to play nice
    fn do_dma(&mut self, src: u8) {
        let addr = (src as u16) << 8;
        for i in 0..=0x9F {
            self.write_u8(0xFE00 + i, self.read_u8(addr + i));
        }
    }
}
