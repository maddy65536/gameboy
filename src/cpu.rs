use serde::Deserialize;

use crate::bus::Bus;

// instruction timings in T-cycles
#[rustfmt::skip]
const INSTRUCTION_TIMINGS: [usize; 256] = [
// +0  +1  +2  +3  +4  +5  +6  +7  +8  +9  +A  +B  +C  +D  +E  +F
    4,  12, 8,  8,  4,  4,  8,  4,  20, 8,  8,  8,  4,  4,  8,  4,  // 0x00
    4,  12, 8,  8,  4,  4,  8,  4,  12, 8,  8,  8,  4,  4,  8,  4,  // 0x10
    8,  12, 8,  8,  4,  4,  8,  4,  8,  8,  8,  8,  4,  4,  8,  4,  // 0x20
    8,  12, 8,  8,  12, 12, 12, 4,  8,  8,  8,  8,  4,  4,  8,  4,  // 0x30
    4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  // 0x40
    4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  // 0x50
    4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  // 0x60
    8,  8,  8,  8,  8,  8,  4,  8,  4,  4,  4,  4,  4,  4,  8,  4,  // 0x70
    4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  // 0x80
    4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  // 0x90
    4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  // 0xA0
    4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,  // 0xB0
    8,  12, 12, 16, 12, 16, 8,  16, 8,  16, 12, 4,  12, 24, 8,  16, // 0xC0
    8,  12, 12, 0,  12, 16, 8,  16, 8,  16, 12, 0,  12, 0,  8,  16, // 0xD0
    12, 12, 8,  0,  0,  16, 8,  16, 16, 4,  16, 0,  0,  0,  8,  16, // 0xE0
    12, 12, 8,  4,  0,  16, 8,  16, 12, 8,  16, 4,  0,  0,  8,  16, // 0xF0
];

// insturction timings with branch in T-cycles
#[rustfmt::skip]
const INSTRUCTION_TIMINGS_BRANCH: [usize; 256] = [
// +0  +1  +2  +3  +4  +5  +6  +7  +8  +9  +A  +B  +C  +D  +E  +F
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0x00
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0x10
    12, 0,  0,  0,  0,  0,  0,  0,  12, 0,  0,  0,  0,  0,  0,  0,  // 0x20
    12, 0,  0,  0,  0,  0,  0,  0,  12, 0,  0,  0,  0,  0,  0,  0,  // 0x30
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0x40
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0x50
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0x60
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0x70
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0x80
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0x90
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0xA0
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0xB0
    20, 0,  16, 0,  24, 0,  0,  0,  20, 0,  16, 0,  24, 0,  0,  0,  // 0xC0
    20, 0,  16, 0,  24, 0,  0,  0,  20, 0,  16, 0,  24, 0,  0,  0,  // 0xD0
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0xE0
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  // 0xF0
];

// CB prefixed instruction timings in T-cycles
#[rustfmt::skip]
const CB_INSTRUCTION_TIMINGS: [usize; 256] = [
// +0  +1  +2  +3  +4  +5  +6  +7  +8  +9  +A  +B  +C  +D  +E  +F
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0x00
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0x10
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0x20
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0x30
    8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8,  12, 8,  // 0x40
    8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8,  12, 8,  // 0x50
    8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8,  12, 8,  // 0x60
    8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8,  12, 8,  // 0x70
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0x80
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0x90
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0xA0
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0xB0
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0xC0
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0xD0
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0xE0
    8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8,  16, 8,  // 0xF0
];

#[derive(Debug)]
pub struct Cpu {
    rf: RegisterFile,
    bus: Bus,
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

enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum Reg16 {
    Af,
    Bc,
    De,
    Hl,
    Sp,
    Pc,
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

    fn write_reg8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::A => self.a = val,
            Reg8::F => self.f = val,
            Reg8::B => self.b = val,
            Reg8::C => self.c = val,
            Reg8::D => self.d = val,
            Reg8::E => self.e = val,
            Reg8::H => self.h = val,
            Reg8::L => self.l = val,
        }
    }

    fn read_reg8(&mut self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::F => self.f,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.f,
            Reg8::L => self.l,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            rf: RegisterFile::default(),
            bus: Bus::new(),
            ime: false,
        }
    }

    pub fn from_state(state: &State) -> Self {
        let mut rf = RegisterFile::default();
        let mut bus = Bus::new();

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
            bus.write_u8(addr, val);
        }

        Self {
            rf,
            bus,
            ime: false,
        }
    }

    pub fn reset(&mut self) {
        self.rf.a = 0;
        self.rf.f = 0;
        self.rf.b = 0;
        self.rf.c = 0;
        self.rf.d = 0;
        self.rf.e = 0;
        self.rf.h = 0;
        self.rf.l = 0;
        self.rf.sp = 0;
        self.rf.pc = 0;
        self.bus.reset();
    }

    pub fn set_state(&mut self, state: &State) {
        self.rf.a = state.a;
        self.rf.f = state.f;
        self.rf.b = state.b;
        self.rf.c = state.c;
        self.rf.d = state.d;
        self.rf.e = state.e;
        self.rf.h = state.h;
        self.rf.l = state.l;
        self.rf.sp = state.sp;
        self.rf.pc = state.pc;

        for (addr, val) in state.ram.iter().cloned() {
            self.bus.write_u8(addr, val);
        }
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
                .bus
                .ram
                .iter()
                .cloned()
                .enumerate()
                .map(|x| (x.0 as u16, x.1))
                .filter(|x| self.bus.touched.contains(&x.0))
                .collect(),
        }
    }

    pub fn fetch_u8(&mut self) -> u8 {
        let res = self.bus.read_u8(self.rf.pc);
        self.rf.pc = self.rf.pc.wrapping_add(1);
        res
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let res = self.bus.read_u16(self.rf.pc);
        self.rf.pc = self.rf.pc.wrapping_add(2);
        res
    }

    pub fn execute_instruction(&mut self) -> usize {
        let opcode = self.fetch_u8();

        match opcode {
            0x00 => (), // NOP
            0x01 | 0x11 | 0x21 | 0x31 => self.ld_r16_imm(opcode),
            0x02 | 0x12 | 0x22 | 0x32 | 0x0A | 0x1A | 0x2A | 0x3A => self.ld_r16ind_a(opcode),
            0x03 | 0x13 | 0x23 | 0x33 => self.inc16(opcode),
            0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C => self.inc8(opcode),
            0x0B | 0x1B | 0x2B | 0x3B => self.dec16(opcode),
            0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D => self.dec8(opcode),
            _ => unimplemented!(
                "unimplemented opcode {:#04X} at pc {:#06X}",
                opcode,
                self.rf.pc - 1
            ),
        }

        INSTRUCTION_TIMINGS[opcode as usize]
    }

    // load a 16 bit register from an immediate
    fn ld_r16_imm(&mut self, opcode: u8) {
        let imm = self.fetch_u16();
        match opcode {
            0x01 => self.rf.write_bc(imm),
            0x11 => self.rf.write_de(imm),
            0x21 => self.rf.write_hl(imm),
            0x31 => self.rf.sp = imm,
            _ => panic!("called ld_imm_16 with unsupported opcode {:#04X}", opcode),
        }
    }

    // load or store from accumulator at an address in a 16 bit register
    fn ld_r16ind_a(&mut self, opcode: u8) {
        match opcode {
            0x02 => self.bus.write_u8(self.rf.read_bc(), self.rf.a),
            0x12 => self.bus.write_u8(self.rf.read_de(), self.rf.a),
            0x22 => {
                self.bus.write_u8(self.rf.read_hl(), self.rf.a);
                self.rf.write_hl(self.rf.read_hl() + 1);
            }
            0x32 => {
                self.bus.write_u8(self.rf.read_hl(), self.rf.a);
                self.rf.write_hl(self.rf.read_hl() - 1);
            }
            0x0A => self.rf.a = self.bus.read_u8(self.rf.read_bc()),
            0x1A => self.rf.a = self.bus.read_u8(self.rf.read_de()),
            0x2A => {
                self.rf.a = self.bus.read_u8(self.rf.read_hl());
                self.rf.write_hl(self.rf.read_hl() + 1);
            }
            0x3A => {
                self.rf.a = self.bus.read_u8(self.rf.read_hl());
                self.rf.write_hl(self.rf.read_hl() - 1);
            }
            _ => panic!("called ld_r16ind_a with unsupported opcode {:#04X}", opcode),
        }
    }

    fn inc16(&mut self, opcode: u8) {
        match opcode {
            0x03 => self.rf.write_bc(self.rf.read_bc().wrapping_add(1)),
            0x13 => self.rf.write_de(self.rf.read_de().wrapping_add(1)),
            0x23 => self.rf.write_hl(self.rf.read_hl().wrapping_add(1)),
            0x33 => self.rf.sp = self.rf.sp.wrapping_add(1),
            _ => panic!("called inc16 with unsupported opcode {:#04X}", opcode),
        }
    }

    fn inc(&mut self, val: u8) -> u8 {
        let res = val.wrapping_add(1);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h((val & 0xF) + 1 > 0xF);
        res
    }

    fn inc8(&mut self, opcode: u8) {
        match opcode {
            0x04 => self.rf.b = self.inc(self.rf.b),
            0x14 => self.rf.d = self.inc(self.rf.d),
            0x24 => self.rf.h = self.inc(self.rf.h),
            0x34 => {
                let val = self.inc(self.bus.read_u8(self.rf.read_hl()));
                self.bus.write_u8(self.rf.read_hl(), val);
            }
            0x0C => self.rf.c = self.inc(self.rf.c),
            0x1C => self.rf.e = self.inc(self.rf.e),
            0x2C => self.rf.l = self.inc(self.rf.l),
            0x3C => self.rf.a = self.inc(self.rf.a),
            _ => panic!("called inc8 with unsupported opcode {:#04X}", opcode),
        }
    }

    fn dec16(&mut self, opcode: u8) {
        match opcode {
            0x0B => self.rf.write_bc(self.rf.read_bc().wrapping_sub(1)),
            0x1B => self.rf.write_de(self.rf.read_de().wrapping_sub(1)),
            0x2B => self.rf.write_hl(self.rf.read_hl().wrapping_sub(1)),
            0x3B => self.rf.sp = self.rf.sp.wrapping_sub(1),
            _ => panic!("called dec16 with unsupported opcode {:#04X}", opcode),
        }
    }

    fn dec(&mut self, val: u8) -> u8 {
        let res = val.wrapping_sub(1);
        self.rf.write_z(res == 0);
        self.rf.write_n(true);
        self.rf.write_h((val & 0xF) == 0x0);
        res
    }

    fn dec8(&mut self, opcode: u8) {
        match opcode {
            0x05 => self.rf.b = self.dec(self.rf.b),
            0x15 => self.rf.d = self.dec(self.rf.d),
            0x25 => self.rf.h = self.dec(self.rf.h),
            0x35 => {
                let val = self.dec(self.bus.read_u8(self.rf.read_hl()));
                self.bus.write_u8(self.rf.read_hl(), val);
            }
            0x0D => self.rf.c = self.dec(self.rf.c),
            0x1D => self.rf.e = self.dec(self.rf.e),
            0x2D => self.rf.l = self.dec(self.rf.l),
            0x3D => self.rf.a = self.dec(self.rf.a),
            _ => panic!("called dec8 with unsupported opcode {:#04X}", opcode),
        }
    }
}
