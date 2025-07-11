use std::fs;

use crate::{cpu::Cpu, mbc::create_cart};

const CYCLES_PER_FRAME: usize = 4194300 / 60;

pub struct Gameboy {
    pub cpu: Cpu,
    cycles: usize,
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
    pub fn new(rom_path: String) -> Self {
        let rom: Vec<u8> = fs::read(rom_path).unwrap();
        let cart = create_cart(rom, None);
        let mut cpu = Cpu::new(cart);
        cpu.simulate_boot();
        Self { cpu, cycles: 0 }
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
}
