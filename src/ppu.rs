use eframe::egui::Color32;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    state: PpuState,
    vram: [u8; 0x2000],
    pub frame: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

#[derive(Debug)]
pub enum PpuState {
    HBlank,
    OAMScan,
    Draw,
    VBlank,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    LightGrey,
    DarkGrey,
    Black,
}

#[allow(clippy::from_over_into)] // this wouldn't make much sense the other way
impl Into<Color32> for Color {
    fn into(self) -> Color32 {
        match self {
            Color::White => Color32::from_rgb(0xFF, 0xFF, 0xFF),
            Color::LightGrey => Color32::from_rgb(0xAA, 0xAA, 0xAA),
            Color::DarkGrey => Color32::from_rgb(0x55, 0x55, 0x55),
            Color::Black => Color32::from_rgb(0x00, 0x00, 0x00),
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        let mut frame = [[Color::White; SCREEN_WIDTH]; SCREEN_HEIGHT];
        frame[100][100] = Color::Black;
        frame[101][100] = Color::Black;
        frame[102][100] = Color::Black;
        frame[100][104] = Color::Black;
        frame[101][104] = Color::Black;
        frame[102][104] = Color::Black;
        frame[105][105] = Color::Black;
        frame[106][104] = Color::Black;
        frame[106][103] = Color::Black;
        frame[105][102] = Color::Black;
        frame[106][101] = Color::Black;
        frame[106][100] = Color::Black;
        frame[105][99] = Color::Black;

        Self {
            state: PpuState::OAMScan,
            vram: [0; 0x2000],
            frame,
        }
    }

    fn read_u8(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write_u8(&mut self, addr: u16, val: u8) {
        todo!()
    }

    fn tick(&mut self, cycles: usize) {
        todo!()
    }
}
