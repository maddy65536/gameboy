pub struct Ppu {
    state: PpuState,
    vram: [u8; 0x2000],
}

pub enum PpuState {
    HBlank,
    OAMScan,
    Draw,
    VBlank,
}

impl Ppu {
    fn new() -> Self {
        Self {
            state: PpuState::OAMScan,
            vram: [0; 0x2000],
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
