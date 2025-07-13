use eframe::egui::Color32;
use proc_bitfield::bitfield;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    state: PpuState,
    vram: [u8; 0x2000],
    tileset: [Tile; 384],
    oam: [u8; 0xA0],
    objects: [Object; 40],
    pub frame: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
    frame_buffer: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
    cycles: u64,
    pub stat_int: bool,
    pub vblank_int: bool,

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
    wly: u8, // for counting window lines
}

#[derive(Debug, PartialEq, Eq)]
pub enum PpuState {
    HBlank = 0,
    VBlank = 1,
    OAMScan = 2,
    Draw = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TilePixel {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixel; 8]; 8];

#[derive(Debug, Clone, Copy)]
struct Object {
    y: u8,
    x: u8,
    tile: u8,
    flags: ObjFlags,
}

fn pixel_to_color(pixel: TilePixel, palette: u8) -> Color {
    match (palette >> ((pixel as u8) << 1)) & 0x03 {
        0 => Color::White,
        1 => Color::LightGrey,
        2 => Color::DarkGrey,
        3 => Color::Black,
        _ => unreachable!(),
    }
}

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

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ObjFlags(pub u8): Debug, FromStorage, IntoStorage, DerefStorage {
        pub priority: bool @ 7,
        pub y_flip: bool @ 6,
        pub x_flip: bool @ 5,
        pub dmg_palette: bool @ 4,
        pub bank: bool @ 3, // CGB only
        pub cgb_palette: u8 @ 0..=2, // CBG only
    }
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            state: PpuState::OAMScan,
            vram: [0; 0x2000],
            tileset: [[[TilePixel::Zero; 8]; 8]; 384],
            oam: [0; 0xA0],
            objects: [Object {
                y: 0,
                x: 0,
                tile: 0,
                flags: 0.into(),
            }; 40],
            frame: [[Color::White; SCREEN_WIDTH]; SCREEN_HEIGHT],
            frame_buffer: [[Color::White; SCREEN_WIDTH]; SCREEN_HEIGHT],
            cycles: 0,
            stat_int: false,
            vblank_int: false,

            lcdc: 0b10100011.into(),
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
            wly: 0,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.read_vram(addr),
            0xFE00..=0xFE9F => self.read_oam(addr),
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
            0xFE00..=0xFE9F => self.write_oam(addr, val),
            0xFF40 => self.lcdc = val.into(),
            0xFF41 => self.stat = val.into(),
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => (), // can't write to ly, do nothing
            0xFF45 => self.set_lyc(val),
            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,
            _ => unreachable!("tried to write to bad region of ppu"),
        }
    }

    fn set_ly(&mut self, val: u8) {
        self.ly = val;
        self.stat.set_lyc_eq_ly(self.ly == self.lyc);
        if self.ly == self.lyc && self.stat.lyc_int_select() {
            self.stat_int = true;
        }
    }

    fn set_lyc(&mut self, val: u8) {
        self.lyc = val;
        self.stat.set_lyc_eq_ly(self.ly == self.lyc);
        if self.ly == self.lyc && self.stat.lyc_int_select() {
            self.stat_int = true;
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

            let val = match (msb, lsb) {
                (0, 0) => TilePixel::Zero,
                (0, _) => TilePixel::One,
                (_, 0) => TilePixel::Two,
                (_, _) => TilePixel::Three,
            };

            self.tileset[tile_index][row_index][pixel_index] = val;
        }
    }

    fn read_oam(&self, addr: u16) -> u8 {
        self.oam[(addr - 0xFE00) as usize]
    }

    fn write_oam(&mut self, addr: u16, val: u8) {
        let index = (addr - 0xFE00) as usize;
        self.oam[index] = val;

        let obj_index = index / 4;
        match index % 4 {
            0 => self.objects[obj_index].y = val,
            1 => self.objects[obj_index].x = val,
            2 => self.objects[obj_index].tile = val,
            3 => self.objects[obj_index].flags = val.into(),
            _ => unreachable!(),
        }
    }

    fn index_to_tile(&self, id: u8, from_lower: bool) -> &Tile {
        if from_lower {
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

    fn get_tileid(&self, x: u8, y: u8, from_upper: bool) -> u8 {
        if from_upper {
            self.vram[0x1C00 + (x as usize) + (y as usize) * 32]
        } else {
            self.vram[0x1800 + (x as usize) + (y as usize) * 32]
        }
    }

    pub fn tick(&mut self, cycles: u64) {
        if !self.lcdc.lcd_ppu_enable() {
            self.stat.set_ppu_mode(0);
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
                    self.set_ly(self.ly + 1);
                    if self.ly >= 144 {
                        self.change_state(PpuState::VBlank);
                    } else {
                        self.change_state(PpuState::OAMScan);
                    }
                }
            }
            PpuState::VBlank => {
                if self.cycles >= 456 {
                    self.cycles -= 456;
                    self.set_ly(self.ly + 1);
                    if self.ly > 153 {
                        self.set_ly(0);
                        self.wly = 0; //reset window line counter
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
                std::mem::swap(&mut self.frame, &mut self.frame_buffer); // does this swap pointers?
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
        if self.lcdc.bg_window_enable() {
            self.draw_bg();
        }
        if self.lcdc.obj_enable() {
            self.draw_obj();
        }
    }

    fn draw_bg(&mut self) {
        let scroll_x = self.scx;
        let scroll_y = self.scy;
        let win_x = self.wx.wrapping_sub(7);
        let win_y = self.wy;

        let tileset = self.lcdc.bg_window_tiles();
        let bg_tilemap = self.lcdc.bg_tilemap();
        let win_tilemap = self.lcdc.window_tilemap();
        let use_window = self.lcdc.window_enable() && self.ly >= win_y;

        let mut pixel;
        let mut window_rendered = false;

        for line_x in 0..160 {
            let x;
            let y;
            let tilemap;

            if use_window && (line_x >= win_x) {
                // render window
                window_rendered = true;
                x = line_x - win_x;
                y = self.wly;
                tilemap = win_tilemap;
            } else {
                // render background
                x = line_x.wrapping_add(scroll_x);
                y = self.ly.wrapping_add(scroll_y);
                tilemap = bg_tilemap;
            }

            let map_x = x / 8;
            let tile_x = x % 8;
            let map_y = y / 8;
            let tile_y = y % 8;

            let tile_id = self.get_tileid(map_x, map_y, tilemap);
            let tile = self.index_to_tile(tile_id, tileset);
            pixel = tile[tile_y as usize][tile_x as usize];
            self.frame_buffer[self.ly as usize][line_x as usize] = pixel_to_color(pixel, self.bgp);
        }
        if window_rendered {
            self.wly += 1;
        }
    }

    fn draw_obj(&mut self) {
        let ly = self.ly as i16; // makes the math easier
        let obj_height = if self.lcdc.obj_size() { 16 } else { 8 };

        // objects to be rendered on this line, max 10
        let mut line_objs: Vec<&Object> = vec![];
        for obj in self.objects.iter() {
            let obj_y = (obj.y as i16) - 16;
            if ly >= obj_y && ly < obj_y + obj_height {
                line_objs.push(obj);
                if line_objs.len() >= 10 {
                    break;
                }
            }
        }

        for line_x in 0..160 {
            // find list of objects to try drawing
            let mut obj_list = vec![];
            for obj in line_objs.iter() {
                let obj_x = (obj.x as i16) - 8;
                if line_x >= obj_x && line_x < obj_x + 8 {
                    obj_list.push(*obj);
                }
            }

            obj_list.sort_by_key(|o| o.x);
            let mut pixel = TilePixel::Zero;
            let mut palette = 0;

            // draw object if there is one
            for obj in obj_list {
                // only draw if it's not set to priority or if it can draw when set to priority
                if !obj.flags.priority()
                    || self.frame_buffer[self.ly as usize][line_x as usize] == Color::White
                {
                    let obj_x = (obj.x as i16) - 8;
                    let obj_y = (obj.y as i16) - 16;
                    let mut tile_x = line_x - obj_x;
                    let mut tile_y = ly - obj_y;

                    // flip tile if necessary
                    if obj.flags.x_flip() {
                        tile_x = 7 - tile_x;
                    }
                    if obj.flags.y_flip() {
                        tile_y = (obj_height - 1) - tile_y;
                    }

                    let tile = if self.lcdc.obj_size() {
                        let tile_ind = obj.tile & !1;
                        if tile_y > 7 {
                            tile_y -= 8;
                            self.index_to_tile(tile_ind + 1, true)
                        } else {
                            self.index_to_tile(tile_ind, true)
                        }
                    } else {
                        self.index_to_tile(obj.tile, true)
                    };

                    palette = if obj.flags.dmg_palette() {
                        self.obp1
                    } else {
                        self.obp0
                    };

                    pixel = tile[tile_y as usize][tile_x as usize];
                    if pixel != TilePixel::Zero {
                        break;
                    }
                }
            }

            if pixel != TilePixel::Zero {
                self.frame_buffer[self.ly as usize][line_x as usize] =
                    pixel_to_color(pixel, palette);
            }
        }
    }
}
