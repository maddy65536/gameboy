use clap::Parser;
use eframe::NativeOptions;
use eframe::egui::ViewportBuilder;
use std::path::Path;

use crate::gameboy::Gameboy;
use crate::gui::GUI_SCALE;
use crate::gui::Gui;
use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

mod apu;
mod bus;
mod cpu;
mod gameboy;
mod gui;
mod joypad;
mod mbc;
mod ppu;
mod serial;
mod timer;

#[derive(Debug, Parser)]
struct Args {
    rom_path: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.rom_path);
    let gb = Gameboy::new(path);
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([
            (SCREEN_WIDTH * GUI_SCALE) as f32,
            (SCREEN_HEIGHT * GUI_SCALE) as f32,
        ]),
        vsync: true,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "meow",
        native_options,
        Box::new(|cc| Ok(Box::new(Gui::new(cc, gb)))),
    );
}
