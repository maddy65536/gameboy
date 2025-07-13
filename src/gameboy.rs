use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    cpu::Cpu,
    mbc::create_cart,
    ppu::{Color, SCREEN_HEIGHT, SCREEN_WIDTH},
};

pub const CLOCK_SPEED: u64 = 4194300;
const CYCLES_PER_FRAME: u64 = CLOCK_SPEED / 60;

pub struct Gameboy {
    pub cpu: Cpu,
    cycles: u64,
    save_path: PathBuf,
}

#[derive(Debug)]
pub enum GbBtn {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Select,
    Start,
}

impl Gameboy {
    pub fn new(rom_path: &Path) -> Self {
        let save_path = rom_path.with_extension("sav");
        let rom: Vec<u8> = fs::read(rom_path).unwrap();
        let save = fs::read(&save_path).ok();
        let cart = create_cart(rom, save);
        let mut cpu = Cpu::new(cart);
        cpu.simulate_boot();
        Self {
            cpu,
            cycles: 0,
            save_path,
        }
    }

    pub fn run_frame(&mut self) {
        while self.cycles < CYCLES_PER_FRAME {
            let cycles = self.cpu.tick();
            self.cycles += cycles;
        }
        self.cycles -= CYCLES_PER_FRAME;
    }

    pub fn set_button(&mut self, button: GbBtn, down: bool) {
        match button {
            GbBtn::Up => self.cpu.bus.joypad.up = down,
            GbBtn::Down => self.cpu.bus.joypad.down = down,
            GbBtn::Left => self.cpu.bus.joypad.left = down,
            GbBtn::Right => self.cpu.bus.joypad.right = down,
            GbBtn::A => self.cpu.bus.joypad.a = down,
            GbBtn::B => self.cpu.bus.joypad.b = down,
            GbBtn::Select => self.cpu.bus.joypad.select = down,
            GbBtn::Start => self.cpu.bus.joypad.start = down,
        }
    }

    pub fn get_frame(&self) -> &[[Color; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        &self.cpu.bus.ppu.frame
    }
}

impl Drop for Gameboy {
    fn drop(&mut self) {
        if self.cpu.bus.cart.has_battery() {
            fs::write(&self.save_path, self.cpu.bus.cart.dump_ram()).unwrap();
        }
    }
}
