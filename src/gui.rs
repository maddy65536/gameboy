use eframe::egui::TextureOptions;
use eframe::egui::{self, ColorImage};
use eframe::egui::{Color32, TextureHandle, widgets::Image};

use crate::Gameboy;
use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub const GUI_SCALE: usize = 4;

pub struct Gui {
    gb: Gameboy,
    screen: TextureHandle,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext<'_>, gb: Gameboy) -> Self {
        let screen = cc.egui_ctx.load_texture(
            "screen",
            egui::ColorImage::new([SCREEN_WIDTH, SCREEN_HEIGHT], Color32::WHITE),
            TextureOptions::NEAREST,
        );
        Self { gb, screen }
    }

    fn update_screen(&mut self) {
        let mut pixels = vec![Color32::WHITE; SCREEN_WIDTH * SCREEN_HEIGHT];
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                pixels[y * SCREEN_WIDTH + x] = self.gb.cpu.bus.ppu.frame[y][x].into();
            }
        }

        self.screen.set(
            ColorImage {
                size: [SCREEN_WIDTH, SCREEN_HEIGHT],
                pixels,
            },
            TextureOptions::NEAREST,
        );
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.gb.run_frame();

        self.update_screen();

        egui::CentralPanel::default().show(ctx, |ui| {
            let image = Image::new(&self.screen);
            image.paint_at(ui, ui.ctx().screen_rect());
        });
    }
}
