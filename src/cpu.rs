#[derive(Debug, Default)]
pub struct RegisterFile {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl RegisterFile {
    // r/w 16-bit registers
    pub fn write_af(&mut self, val: u16) {
        self.a = ((val & 0xFF00) >> 8) as u8;
        self.f = (val & 0x00FF) as u8;
    }

    pub fn read_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn write_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val & 0x00FF) as u8;
    }

    pub fn read_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn write_de(&mut self, val: u16) {
        self.d = ((val & 0xFF00) >> 8) as u8;
        self.e = (val & 0x00FF) as u8;
    }

    pub fn read_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn write_hl(&mut self, val: u16) {
        self.h = ((val & 0xFF00) >> 8) as u8;
        self.l = (val & 0x00FF) as u8;
    }

    pub fn read_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    // r/w flags
    pub fn write_z(&mut self, val: bool) {
        self.f = (self.f & !(1 << 7)) | ((val as u8) << 7);
    }

    pub fn read_z(&self) -> bool {
        ((self.f & (1 << 7)) >> 7) == 1
    }

    pub fn write_n(&mut self, val: bool) {
        self.f = (self.f & !(1 << 6)) | ((val as u8) << 6);
    }

    pub fn read_n(&self) -> bool {
        ((self.f & (1 << 6)) >> 6) == 1
    }

    pub fn write_h(&mut self, val: bool) {
        self.f = (self.f & !(1 << 5)) | ((val as u8) << 5);
    }

    pub fn read_h(&self) -> bool {
        ((self.f & (1 << 5)) >> 5) == 1
    }

    pub fn write_c(&mut self, val: bool) {
        self.f = (self.f & !(1 << 4)) | ((val as u8) << 4);
    }

    pub fn read_c(&self) -> bool {
        ((self.f & (1 << 4)) >> 4) == 1
    }
}
