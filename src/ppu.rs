use eframe::egui::Color32;
use proc_bitfield::bitfield;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    state: PpuState,
    vram: [u8; 0x2000],
    tileset: [Tile; 384],
    pub frame: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
    frame_buffer: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
    cycles: usize,
    pub stat_int: bool,
    pub vblank_int: bool,

    // just doing this for now
    lcdc: Lcdc,
    stat: Stat,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PpuState {
    HBlank = 0,
    VBlank = 1,
    OAMScan = 2,
    Draw = 3,
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

// tileset cache based on https://rylev.github.io/DMG-01/public/book/graphics/tile_ram.html
#[derive(Debug, Clone, Copy)]
enum TilePixel {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixel; 8]; 8];

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Lcdc(pub u8): Debug, FromStorage, IntoStorage, DerefStorage {
        pub lcd_ppu_enable: bool @ 7,
        pub window_tilemap: bool @ 6,
        pub window_enable: bool @ 5,
        pub bg_window_tiles: bool @ 4,
        pub bg_tilemap: bool @ 3,
        pub obj_size: bool @ 2,
        pub obj_enable: bool @ 1,
        pub bg_window_enable: bool @ 0,
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Stat(pub u8): Debug, FromStorage, IntoStorage, DerefStorage {
        pub lyc_int_select: bool @ 6,
        pub mode_2_int_select: bool @ 5,
        pub mode_1_int_select: bool @ 4,
        pub mode_0_int_select: bool @ 3,
        pub lyc_eq_ly: bool @ 2,
        pub ppu_mode: u8 @ 0..=1,
    }
}

impl Ppu {
    pub fn new() -> Self {
        let mut frame_buffer = [[Color::White; SCREEN_WIDTH]; SCREEN_HEIGHT];

        Self {
            state: PpuState::OAMScan,
            vram: [0; 0x2000],
            tileset: [[[TilePixel::Zero; 8]; 8]; 384],
            frame: [[Color::White; SCREEN_WIDTH]; SCREEN_HEIGHT],
            frame_buffer,
            cycles: 0,
            stat_int: false,
            vblank_int: false,

            lcdc: 0.into(),
            stat: 0.into(),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.read_vram(addr),
            0xFF40 => self.lcdc.into(),
            0xFF41 => self.stat.into(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
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
            0x8000..=0x9FFF => self.write_vram(addr, val),
            0xFF40 => {
                self.lcdc = val.into();
                // is this how it works?
                if !self.lcdc.lcd_ppu_enable() {
                    self.ly = 0;
                    self.change_state(PpuState::OAMScan);
                }
            }
            0xFF41 => self.stat = val.into(),
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => (), // can't write to ly, do nothing
            0xFF45 => self.lyc = val,
            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,
            _ => unreachable!("tried to write to bad region of ppu"),
        }
    }

    fn read_vram(&self, addr: u16) -> u8 {
        // only ever called behind a check
        self.vram[(addr - 0x8000) as usize]
    }

    fn write_vram(&mut self, addr: u16, val: u8) {
        let index = (addr - 0x8000) as usize;
        self.vram[index] = val;

        // outside tileset?
        if index >= 0x1800 {
            return;
        }

        let norm_index = index & 0xFFFE;
        let b1 = self.vram[norm_index];
        let b2 = self.vram[norm_index + 1];
        let tile_index = index / 16;
        let row_index = (index % 16) / 2;

        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);
            let lsb = b1 & mask;
            let msb = b2 & mask;

            let val = match (lsb, msb) {
                (0, 0) => TilePixel::Zero,
                (0, _) => TilePixel::One,
                (_, 0) => TilePixel::Two,
                (_, _) => TilePixel::Three,
            };

            self.tileset[tile_index][row_index][pixel_index] = val;
        }
    }

    fn index_to_tile(&self, id: u8, from_upper: bool) -> &Tile {
        if !from_upper {
            // 0x8000 method
            &self.tileset[id as usize]
        } else {
            // 0x8800 method
            // i could do clever casting stuff here but i don't wanna!
            if id <= 127 {
                &self.tileset[(id as usize) + 256]
            } else {
                &self.tileset[id as usize]
            }
        }
    }

    fn get_tileid_1(&self, x: u8, y: u8) -> u8 {
        self.vram[0x1800 + (x as usize) + (y as usize) * 32]
    }

    fn get_tileid_2(&self, x: u8, y: u8) -> u8 {
        self.vram[0x1C00 + (x as usize) + (y as usize) * 32]
    }

    pub fn tick(&mut self, cycles: usize) {
        if !self.lcdc.lcd_ppu_enable() {
            return;
        }
        self.cycles += cycles;
        match self.state {
            PpuState::OAMScan => {
                if self.cycles >= 80 {
                    self.cycles -= 80;
                    self.change_state(PpuState::Draw);
                }
            }
            PpuState::Draw => {
                // technically variable but i don't care for now lol
                if self.cycles >= 172 {
                    self.cycles -= 172;
                    self.change_state(PpuState::HBlank);
                }
            }
            PpuState::HBlank => {
                if self.cycles >= 204 {
                    self.cycles -= 204;
                    self.ly += 1;
                    if self.ly == 144 {
                        self.change_state(PpuState::VBlank);
                    } else {
                        self.change_state(PpuState::OAMScan);
                    }
                }
            }
            PpuState::VBlank => {
                if self.cycles >= 456 {
                    self.cycles -= 456;
                    self.ly += 1;
                    if self.ly > 153 {
                        self.ly = 0;
                        self.change_state(PpuState::OAMScan);
                    }
                }
            }
        }
    }

    fn change_state(&mut self, state: PpuState) {
        match state {
            PpuState::HBlank => {
                self.draw_line();

                self.state = PpuState::HBlank;
                self.stat.set_ppu_mode(PpuState::HBlank as u8);

                if self.stat.mode_0_int_select() {
                    self.stat_int = true;
                }
            }
            PpuState::VBlank => {
                // move frame buffer onto application window and clear frame buffer
                std::mem::swap(&mut self.frame, &mut self.frame_buffer);
                self.frame_buffer.fill([Color::White; SCREEN_WIDTH]);

                self.state = PpuState::VBlank;
                self.stat.set_ppu_mode(PpuState::VBlank as u8);

                self.vblank_int = true;
                if self.stat.mode_1_int_select() {
                    self.stat_int = true;
                }
            }
            PpuState::OAMScan => {
                self.state = PpuState::OAMScan;
                self.stat.set_ppu_mode(PpuState::OAMScan as u8);

                if self.stat.mode_2_int_select() {
                    self.stat_int = true;
                }
            }
            PpuState::Draw => {
                self.state = PpuState::Draw;
                self.stat.set_ppu_mode(PpuState::Draw as u8);
            }
        }
    }

    fn draw_line(&mut self) {
        self.draw_bg();
        self.draw_obj();
    }

    fn draw_bg(&mut self) {
        // draw bg

        // draw window
    }

    fn draw_obj(&mut self) {}
}
