use eframe::egui::TextureOptions;
use eframe::egui::{self, ColorImage};
use eframe::egui::{Color32, TextureHandle, widgets::Image};
use std::time::{Duration, Instant};

use crate::Gameboy;
use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub const GUI_SCALE: usize = 4;
pub const FPS: f64 = 60.;

pub struct Gui {
    gb: Gameboy,
    screen: TextureHandle,
    last_frame: Instant,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext<'_>, gb: Gameboy) -> Self {
        let screen = cc.egui_ctx.load_texture(
            "screen",
            egui::ColorImage::new([SCREEN_WIDTH, SCREEN_HEIGHT], Color32::WHITE),
            TextureOptions::NEAREST,
        );
        Self {
            gb,
            screen,
            last_frame: Instant::now(),
        }
    }

    fn update_screen(&mut self) {
        let mut pixels = vec![Color32::WHITE; SCREEN_WIDTH * SCREEN_HEIGHT];
        let frame = self.gb.get_frame();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                pixels[y * SCREEN_WIDTH + x] = frame[y][x].into();
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

    #[rustfmt::skip]
    fn update_input(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            self.gb.set_button(crate::gameboy::GbBtn::Up,     i.key_down(egui::Key::ArrowUp));
            self.gb.set_button(crate::gameboy::GbBtn::Down,   i.key_down(egui::Key::ArrowDown));
            self.gb.set_button(crate::gameboy::GbBtn::Left,   i.key_down(egui::Key::ArrowLeft));
            self.gb.set_button(crate::gameboy::GbBtn::Right,  i.key_down(egui::Key::ArrowRight));
            self.gb.set_button(crate::gameboy::GbBtn::A,      i.key_down(egui::Key::X));
            self.gb.set_button(crate::gameboy::GbBtn::B,      i.key_down(egui::Key::Z));
            self.gb.set_button(crate::gameboy::GbBtn::Select, i.key_down(egui::Key::A));
            self.gb.set_button(crate::gameboy::GbBtn::Start,  i.key_down(egui::Key::S));
        })
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame_goal = Duration::from_secs_f64(1.0 / FPS);
        let now = Instant::now();
        //println!("fps: {}", 1.0 / (now - self.last_frame).as_secs_f64());
        self.last_frame = now;

        self.update_input(ctx);

        self.gb.run_frame();

        self.update_screen();

        egui::CentralPanel::default().show(ctx, |ui| {
            let image = Image::new(&self.screen);
            image.paint_at(ui, ui.ctx().screen_rect());
        });

        let frame_time = now.elapsed();
        ctx.request_repaint_after(frame_goal - frame_time);
        // ctx.request_repaint_after(Duration::from_millis(17));
    }
}
