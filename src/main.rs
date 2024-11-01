use clap::Parser;
use eframe::egui;
use std::env;

use crate::gameboy::Gameboy;
use crate::gui::Gui;

mod bus;
mod cart;
mod cpu;
mod gameboy;
mod gui;
mod ppu;
mod timer;

#[derive(Debug, Parser)]
struct Args {
    rom_path: String,
}

fn main() {
    let args = Args::parse();

    let gb = Gameboy::new(args.rom_path);
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "meow",
        native_options,
        Box::new(|cc| Ok(Box::new(Gui::new(cc, gb)))),
    );
}
