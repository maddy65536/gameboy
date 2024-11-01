use eframe::egui;

use crate::Gameboy;

pub struct Gui {
    gb: Gameboy,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext<'_>, gb: Gameboy) -> Self {
        Self { gb }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.gb.run_frame();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("nothing here yet");
        });
    }
}
