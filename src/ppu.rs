use eframe::egui::Color32;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    state: PpuState,
    vram: [u8; 0x2000],
    pub frame: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
    cycles: usize,

    // just doing this for now
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
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

impl From<Color> for Color32 {
    fn from(value: Color) -> Self {
        match value {
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
            cycles: 0,

            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => unreachable!("tried to read from bad region of ppu"),
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xFF40 => self.lcdc = val,
            0xFF41 => self.stat = val,
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => self.ly = val,
            0xFF45 => self.lyc = val,
            0xFF46 => self.dma = val,
            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,
            _ => unreachable!("tried to write to bad region of ppu"),
        }
    }

    fn tick(&mut self, cycles: usize) {
        self.cycles += cycles;
        match self.state {
            PpuState::OAMScan => todo!("OAM scan"),
            PpuState::Draw => todo!("draw"),
            PpuState::HBlank => todo!("hblank"),
            PpuState::VBlank => todo!("vblank"),
        }
    }
}
