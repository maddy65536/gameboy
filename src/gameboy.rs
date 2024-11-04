use std::fs;

use crate::{cart::Cart, cpu::Cpu};

const CYCLES_PER_FRAME: usize = 4194300 / 60;

pub struct Gameboy {
    pub cpu: Cpu,
    cycles: usize,
}

impl Gameboy {
    pub fn new(rom_path: String) -> Self {
        let rom: Vec<u8> = fs::read(rom_path).unwrap();
        let cart = Cart::new(rom);
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
}
