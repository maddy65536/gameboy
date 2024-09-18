use serde::Deserialize;

use crate::interconnect::Interconnect;

#[derive(Debug)]
pub struct Cpu {
    rf: RegisterFile,
    ic: Interconnect,
    ime: bool,
}

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

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct State {
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
    pub ram: Vec<(u16, u8)>,
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

impl Cpu {
    pub fn new() -> Self {
        Self {
            rf: RegisterFile::default(),
            ic: Interconnect::new(),
            ime: false,
        }
    }

    pub fn from_state(state: &State) -> Self {
        let mut rf = RegisterFile::default();
        let mut ic = Interconnect::new();

        rf.a = state.a;
        rf.f = state.f;
        rf.b = state.b;
        rf.c = state.c;
        rf.d = state.d;
        rf.e = state.e;
        rf.h = state.h;
        rf.l = state.l;
        rf.sp = state.sp;
        rf.pc = state.pc;

        for (addr, val) in state.ram.iter().cloned() {
            ic.write_u8(addr, val);
        }

        Self { rf, ic, ime: false }
    }

    pub fn to_state(&self) -> State {
        State {
            a: self.rf.a,
            f: self.rf.f,
            b: self.rf.b,
            c: self.rf.c,
            d: self.rf.d,
            e: self.rf.e,
            h: self.rf.h,
            l: self.rf.l,
            sp: self.rf.sp,
            pc: self.rf.pc,
            ram: self
                .ic
                .ram
                .iter()
                .cloned()
                .enumerate()
                .map(|x| (x.0 as u16, x.1))
                .filter(|x| self.ic.touched.contains(&x.0))
                .collect(),
        }
    }

    pub fn fetch_u8(&mut self) -> u8 {
        let res = self.ic.read_u8(self.rf.pc);
        self.rf.pc += 1;
        res
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let res = self.ic.read_u16(self.rf.pc);
        self.rf.pc += 2;
        res
    }

    pub fn execute_instruction(&mut self) -> usize {
        let opcode = self.fetch_u8();

        match opcode {
            0x00 => 4,
            _ => unimplemented!(
                "unimplemented opcode {:#04X} at pc {:#06X}",
                opcode,
                self.rf.pc - 1
            ),
        }
    }
}
