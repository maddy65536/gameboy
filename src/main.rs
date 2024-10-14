use cart::Cart;
use cpu::Cpu;
use std::env;
use std::fs;

mod bus;
mod cart;
mod cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(":(");
    }

    let rom: Vec<u8> = fs::read(&args[1]).unwrap();
    let mut cpu = Cpu::new(Cart::new(rom));
    cpu.simulate_boot();
    loop {
        cpu.tick();
    }
}
