use crate::bus::Bus;
use crate::cart::Cart;

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
    0,  0,  0,  0,  0,  0,  0,  0,  12, 0,  0,  0,  0,  0,  0,  0,  // 0x10
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
    20, 0,  16, 16, 24, 0,  0,  16, 20, 16, 16, 0,  24, 24, 0,  16, // 0xC0
    20, 0,  16, 0,  24, 0,  0,  16, 20, 16, 16, 0,  24, 0,  0,  16, // 0xD0
    0,  0,  0,  0,  0,  0,  0,  16, 0,  4,  0,  0,  0,  0,  0,  16, // 0xE0
    0,  0,  0,  0,  0,  0,  0,  16, 0,  0,  0,  0,  0,  0,  0,  16, // 0xF0
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

const HL_IND_REG_NUM: u8 = 0x6;

#[derive(Debug)]
pub struct Cpu {
    rf: RegisterFile,
    bus: Bus,
    ime: bool,
    pending_ime: bool,
    halted: bool,
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

impl RegisterFile {
    // r/w 16-bit registers
    pub fn write_af(&mut self, val: u16) {
        self.a = ((val & 0xFF00) >> 8) as u8;
        self.f = (val & 0x00F0) as u8;
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

impl std::ops::Index<u8> for RegisterFile {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.b,
            1 => &self.c,
            2 => &self.d,
            3 => &self.e,
            4 => &self.h,
            5 => &self.l,
            7 => &self.a,
            _ => panic!("cannot convert {} to a register!", index),
        }
    }
}

impl std::ops::IndexMut<u8> for RegisterFile {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.b,
            1 => &mut self.c,
            2 => &mut self.d,
            3 => &mut self.e,
            4 => &mut self.h,
            5 => &mut self.l,
            7 => &mut self.a,
            _ => panic!("cannot convert {} to a register!", index),
        }
    }
}

impl Cpu {
    pub fn new(cart: Cart) -> Self {
        Self {
            rf: RegisterFile::default(),
            bus: Bus::new(cart),
            ime: false,
            pending_ime: false,
            halted: false,
        }
    }

    pub fn simulate_boot(&mut self) {
        self.rf.a = 0x01;
        self.rf.write_z(true);
        self.rf.b = 0x00;
        self.rf.c = 0x13;
        self.rf.d = 0x00;
        self.rf.e = 0xD8;
        self.rf.h = 0x01;
        self.rf.l = 0x4D;
        self.rf.pc = 0x0100;
        self.rf.sp = 0xFFFE;

        self.bus.timer.div = 0xAB;
        self.bus.timer.tima = 0x00;
        self.bus.timer.tma = 0x00;
        self.bus.timer.tac = 0xF8;
    }

    pub fn tick(&mut self) -> usize {
        let icycles = self.handle_interrupts();
        if self.pending_ime {
            self.ime = true;
            self.pending_ime = false;
        }
        let cycles = icycles + self.execute_instruction();
        self.bus.tick(cycles);
        cycles
    }

    fn handle_interrupts(&mut self) -> usize {
        if self.pending_ime {
            self.ime = true;
            self.pending_ime = false;
        }

        let i_flags = self.bus.read_u8(0xFF0F);
        let i_enable = self.bus.read_u8(0xFFFF);

        if i_flags & i_enable != 0 {
            if self.ime {
                let int = i_flags.trailing_zeros(); // is this a clever way to do this? i hope so!
                let addr = match int {
                    0 => 0x0040, // VBlank
                    1 => 0x0048, // LCD
                    2 => 0x0050, // Timer
                    3 => 0x0058, // Serial
                    4 => 0x0060, // Joypad
                    _ => panic!("invalid interrupt??: {}", int),
                };

                self.ime = false;
                self.bus.write_u8(0xFF0F, i_flags & !(1 << int)); // update IF
                self.push_u16(self.rf.pc);
                self.rf.pc = addr
            }

            self.halted = false;
            return 20;
        }

        0
    }

    fn fetch_u8(&mut self) -> u8 {
        let res = self.bus.read_u8(self.rf.pc);
        self.rf.pc = self.rf.pc.wrapping_add(1);
        res
    }

    fn fetch_u16(&mut self) -> u16 {
        let res = self.bus.read_u16(self.rf.pc);
        self.rf.pc = self.rf.pc.wrapping_add(2);
        res
    }

    fn read_hl_ind(&self) -> u8 {
        self.bus.read_u8(self.rf.read_hl())
    }

    fn write_hl_ind(&mut self, val: u8) {
        self.bus.write_u8(self.rf.read_hl(), val);
    }

    fn write_r8(&mut self, reg: u8, val: u8) {
        match reg {
            HL_IND_REG_NUM => self.write_hl_ind(val),
            _ => self.rf[reg] = val,
        }
    }

    fn read_r8(&self, reg: u8) -> u8 {
        match reg {
            HL_IND_REG_NUM => self.read_hl_ind(),
            _ => self.rf[reg],
        }
    }

    fn push_u16(&mut self, val: u16) {
        self.rf.sp = self.rf.sp.wrapping_sub(2);
        self.bus.write_u16(self.rf.sp, val);
    }

    fn pop_u16(&mut self) -> u16 {
        let val = self.bus.read_u16(self.rf.sp);
        self.rf.sp = self.rf.sp.wrapping_add(2);
        val
    }

    pub fn execute_instruction(&mut self) -> usize {
        if self.halted {
            return 4;
        }

        let opcode = self.fetch_u8();
        match opcode {
            //misc
            0x00 => (),                                   // NOP
            0xCB => return self.cb_execute_instruction(), // cb prefixed instructions
            0xF3 => self.di(),                            // DI
            0xFB => self.ei(),                            // EI
            0x10 => self.stop(),                          // STOP
            0x76 => self.halted = true,                   // HALT
            // loads
            0x01 | 0x11 | 0x21 | 0x31 => self.ld_r16_imm(opcode),
            0x06 | 0x16 | 0x26 | 0x36 | 0x0E | 0x1E | 0x2E | 0x3E => self.ld_r8_imm(opcode),
            0x02 | 0x12 | 0x22 | 0x32 | 0x0A | 0x1A | 0x2A | 0x3A => self.ld_ind_a(opcode),
            0x40..=0x75 | 0x77..=0x7F => self.ld_r8(opcode),
            0x08 => self.ld_ind_imm_sp(),
            0xF9 => self.ld_sp(),
            0xC1 | 0xD1 | 0xE1 | 0xF1 => self.pop(opcode),
            0xC5 | 0xD5 | 0xE5 | 0xF5 => self.push(opcode),
            0xF8 => self.ld_hl_sp_offset(),
            0xE0 | 0xF0 | 0xE2 | 0xF2 => self.ld_a_ind_offset(opcode),
            0xEA | 0xFA => self.ld_a_ind(opcode),
            // math
            0x07 => self.rlca(),
            0x17 => self.rla(),
            0x0F => self.rrca(),
            0x1F => self.rra(),
            0x03 | 0x13 | 0x23 | 0x33 => self.inc16(opcode),
            0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C => self.inc8(opcode),
            0x0B | 0x1B | 0x2B | 0x3B => self.dec16(opcode),
            0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D => self.dec8(opcode),
            0x09 | 0x19 | 0x29 | 0x39 => self.add16(opcode),
            0x80..=0x87 | 0xC6 => self.add(opcode),
            0x88..=0x8F | 0xCE => self.adc(opcode),
            0x90..=0x97 | 0xD6 => self.sub(opcode),
            0x98..=0x9F | 0xDE => self.sbc(opcode),
            0xA0..=0xA7 | 0xE6 => self.and(opcode),
            0xA8..=0xAF | 0xEE => self.xor(opcode),
            0xB0..=0xB7 | 0xF6 => self.or(opcode),
            0xB8..=0xBF | 0xFE => self.cp(opcode),
            0x27 => self.daa(),
            0x37 => self.scf(),
            0x2F => self.cpl(),
            0x3F => self.ccf(),
            0xE8 => self.add_sp_imm(),
            // branches
            0x18 | 0x28 | 0x38 | 0x20 | 0x30 => return self.jr(opcode),
            0xC0 | 0xD0 | 0xC8 | 0xD8 | 0xC9 | 0xD9 => return self.ret(opcode),
            0xC2 | 0xD2 | 0xC3 | 0xE9 | 0xCA | 0xDA => return self.jp(opcode),
            0xC4 | 0xD4 | 0xCC | 0xDC | 0xCD => return self.call(opcode),
            0xC7 | 0xD7 | 0xE7 | 0xF7 | 0xCF | 0xDF | 0xEF | 0xFF => self.rst(opcode),
            _ => unimplemented!(
                "unimplemented opcode {:#04X} at pc {:#06X}",
                opcode,
                self.rf.pc - 1,
            ),
        }

        INSTRUCTION_TIMINGS[opcode as usize]
    }

    fn cb_execute_instruction(&mut self) -> usize {
        let opcode = self.fetch_u8();

        match opcode {
            0x00..=0x07 => self.rlc(opcode),
            0x08..=0x0F => self.rrc(opcode),
            0x10..=0x17 => self.rl(opcode),
            0x18..=0x1F => self.rr(opcode),
            0x20..=0x27 => self.sla(opcode),
            0x28..=0x2F => self.sra(opcode),
            0x30..=0x37 => self.swap(opcode),
            0x38..=0x3F => self.srl(opcode),
            0x40..=0x7F => self.bit(opcode),
            0x80..=0xBF => self.res(opcode),
            0xC0..=0xFF => self.set(opcode),
        }

        CB_INSTRUCTION_TIMINGS[opcode as usize]
    }
}

// instructions
impl Cpu {
    // misc
    fn di(&mut self) {
        self.ime = false;
        self.pending_ime = false;
    }

    fn ei(&mut self) {
        if !self.ime {
            self.pending_ime = true;
        }
    }

    fn stop(&mut self) {
        self.bus.timer.reset_divider();
    }

    // loads
    fn ld_r16_imm(&mut self, opcode: u8) {
        let imm = self.fetch_u16();
        match opcode {
            0x01 => self.rf.write_bc(imm),
            0x11 => self.rf.write_de(imm),
            0x21 => self.rf.write_hl(imm),
            0x31 => self.rf.sp = imm,
            _ => panic!("invalid opcode {:#04X} in ld_r16_imm", opcode),
        }
    }

    fn ld_r8_imm(&mut self, opcode: u8) {
        let imm = self.fetch_u8();
        let reg = (opcode >> 3) & 7;
        self.write_r8(reg, imm);
    }

    fn ld_ind_a(&mut self, opcode: u8) {
        match opcode {
            0x02 => self.bus.write_u8(self.rf.read_bc(), self.rf.a),
            0x12 => self.bus.write_u8(self.rf.read_de(), self.rf.a),
            0x22 => {
                self.write_hl_ind(self.rf.a);
                self.rf.write_hl(self.rf.read_hl() + 1);
            }
            0x32 => {
                self.write_hl_ind(self.rf.a);
                self.rf.write_hl(self.rf.read_hl() - 1);
            }
            0x0A => self.rf.a = self.bus.read_u8(self.rf.read_bc()),
            0x1A => self.rf.a = self.bus.read_u8(self.rf.read_de()),
            0x2A => {
                self.rf.a = self.read_hl_ind();
                self.rf.write_hl(self.rf.read_hl() + 1);
            }
            0x3A => {
                self.rf.a = self.read_hl_ind();
                self.rf.write_hl(self.rf.read_hl() - 1);
            }
            _ => panic!("invalid opcode {:#04X} in ld_ind_a", opcode),
        }
    }

    fn ld_r8(&mut self, opcode: u8) {
        let src = opcode & 7;
        let dest = (opcode >> 3) & 7;
        self.write_r8(dest, self.read_r8(src));
    }

    fn ld_ind_imm_sp(&mut self) {
        let addr = self.fetch_u16();
        self.bus.write_u16(addr, self.rf.sp);
    }

    fn ld_sp(&mut self) {
        self.rf.sp = self.rf.read_hl();
    }

    fn pop(&mut self, opcode: u8) {
        let val = self.pop_u16();
        match opcode {
            0xC1 => self.rf.write_bc(val),
            0xD1 => self.rf.write_de(val),
            0xE1 => self.rf.write_hl(val),
            0xF1 => self.rf.write_af(val),
            _ => panic!("invalid opcode {:#04X} in pop", opcode),
        }
    }

    fn push(&mut self, opcode: u8) {
        match opcode {
            0xC5 => self.push_u16(self.rf.read_bc()),
            0xD5 => self.push_u16(self.rf.read_de()),
            0xE5 => self.push_u16(self.rf.read_hl()),
            0xF5 => self.push_u16(self.rf.read_af()),
            _ => panic!("invalid opcode {:#04X} in push", opcode),
        }
    }

    fn ld_hl_sp_offset(&mut self) {
        let a = self.rf.sp;
        let b = self.fetch_u8();

        self.rf.write_z(false);
        self.rf.write_n(false);
        self.rf.write_h((a & 0xF) + ((b as u16) & 0xF) > 0xF);
        self.rf.write_c((a & 0xFF) + (b as u16) > 0xFF);

        self.rf.write_hl(a.wrapping_add_signed((b as i8) as i16));
    }

    fn ld_a_ind_offset(&mut self, opcode: u8) {
        let offset = if opcode == 0xE0 || opcode == 0xF0 {
            self.fetch_u8()
        } else {
            self.rf.c
        } as u16;
        match opcode {
            0xE0 => self.bus.write_u8(0xFF00 + offset, self.rf.a),
            0xF0 => self.rf.a = self.bus.read_u8(0xFF00 + offset),
            0xE2 => self.bus.write_u8(0xFF00 + offset, self.rf.a),
            0xF2 => self.rf.a = self.bus.read_u8(0xFF00 + offset),
            _ => panic!("invalid opcode {:#04X} in la_a_ind_offset", opcode),
        }
    }

    fn ld_a_ind(&mut self, opcode: u8) {
        let addr = self.fetch_u16();
        match opcode {
            0xEA => self.bus.write_u8(addr, self.rf.a),
            0xFA => self.rf.a = self.bus.read_u8(addr),
            _ => panic!("invalid opcode {:#04X} in la_a_ind", opcode),
        }
    }

    // math
    fn rlca(&mut self) {
        let val = self.rf.a;
        let res = val.rotate_left(1);
        self.rf.write_z(false);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x80 == 0x80);
        self.rf.a = res;
    }

    fn rla(&mut self) {
        let val = self.rf.a;
        let res = (val << 1) | (self.rf.read_c() as u8);
        self.rf.write_z(false);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x80 == 0x80);
        self.rf.a = res;
    }

    fn rrca(&mut self) {
        let val = self.rf.a;
        let res = val.rotate_right(1);
        self.rf.write_z(false);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x1 == 0x1);
        self.rf.a = res;
    }

    fn rra(&mut self) {
        let val = self.rf.a;
        let res = (val >> 1) | ((self.rf.read_c() as u8) << 7);
        self.rf.write_z(false);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x1 == 0x1);
        self.rf.a = res;
    }

    fn inc16(&mut self, opcode: u8) {
        match opcode {
            0x03 => self.rf.write_bc(self.rf.read_bc().wrapping_add(1)),
            0x13 => self.rf.write_de(self.rf.read_de().wrapping_add(1)),
            0x23 => self.rf.write_hl(self.rf.read_hl().wrapping_add(1)),
            0x33 => self.rf.sp = self.rf.sp.wrapping_add(1),
            _ => panic!("invalid opcode {:#04X} in inc16", opcode),
        }
    }

    fn inc8(&mut self, opcode: u8) {
        let reg = (opcode >> 3) & 7;
        let val = self.read_r8(reg);

        let res = val.wrapping_add(1);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h((val & 0xF) + 1 > 0xF);

        self.write_r8(reg, res);
    }

    fn dec16(&mut self, opcode: u8) {
        match opcode {
            0x0B => self.rf.write_bc(self.rf.read_bc().wrapping_sub(1)),
            0x1B => self.rf.write_de(self.rf.read_de().wrapping_sub(1)),
            0x2B => self.rf.write_hl(self.rf.read_hl().wrapping_sub(1)),
            0x3B => self.rf.sp = self.rf.sp.wrapping_sub(1),
            _ => panic!("invalid opcode {:#04X} in dec16", opcode),
        }
    }

    fn dec8(&mut self, opcode: u8) {
        let reg = (opcode >> 3) & 7;
        let val = self.read_r8(reg);

        let res = val.wrapping_sub(1);
        self.rf.write_z(res == 0);
        self.rf.write_n(true);
        self.rf.write_h((val & 0xF) == 0x0);

        self.write_r8(reg, res);
    }

    fn add16(&mut self, opcode: u8) {
        let a = self.rf.read_hl();
        let b = match opcode {
            0x09 => self.rf.read_bc(),
            0x19 => self.rf.read_de(),
            0x29 => self.rf.read_hl(),
            0x39 => self.rf.sp,
            _ => panic!("invalid opcode {:#04X} in add16", opcode),
        };

        let res = a.wrapping_add(b);
        self.rf.write_n(false);
        self.rf.write_h((a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF);
        self.rf.write_c(res < a);
        self.rf.write_hl(res);
    }

    fn add(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let a = self.rf.a;
        let b = if opcode == 0xC6 {
            self.fetch_u8()
        } else {
            self.read_r8(reg)
        };

        let res = a.wrapping_add(b);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h((a & 0xF) + (b & 0xF) > 0xF);
        self.rf.write_c((a as u16) + (b as u16) > 0xFF);
        self.rf.a = res;
    }

    fn adc(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let c = self.rf.read_c() as u8;
        let a = self.rf.a;
        let b = if opcode == 0xCE {
            self.fetch_u8()
        } else {
            self.read_r8(reg)
        };

        let res = a.wrapping_add(b).wrapping_add(c);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h((a & 0xF) + (b & 0xF) + c > 0xF);
        self.rf.write_c((a as u16) + (b as u16) + (c as u16) > 0xFF);
        self.rf.a = res;
    }

    fn sub(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let a = self.rf.a;
        let b = if opcode == 0xD6 {
            self.fetch_u8()
        } else {
            self.read_r8(reg)
        };

        let res = a.wrapping_sub(b);
        self.rf.write_z(res == 0);
        self.rf.write_n(true);
        self.rf.write_h((a & 0xF) < (b & 0xF));
        self.rf.write_c(a < b);
        self.rf.a = res;
    }

    fn sbc(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let c = self.rf.read_c() as u8;
        let a = self.rf.a;
        let b = if opcode == 0xDE {
            self.fetch_u8()
        } else {
            self.read_r8(reg)
        };

        let res = a.wrapping_sub(b).wrapping_sub(c);
        self.rf.write_z(res == 0);
        self.rf.write_n(true);
        self.rf.write_h((a & 0xF) < (b & 0xF) + c);
        self.rf.write_c((a as u16) < (b as u16) + (c as u16));
        self.rf.a = res;
    }

    fn and(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let a = self.rf.a;
        let b = if opcode == 0xE6 {
            self.fetch_u8()
        } else {
            self.read_r8(reg)
        };

        let res = a & b;
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(true);
        self.rf.write_c(false);
        self.rf.a = res;
    }

    fn xor(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let a = self.rf.a;
        let b = if opcode == 0xEE {
            self.fetch_u8()
        } else {
            self.read_r8(reg)
        };

        let res = a ^ b;
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(false);
        self.rf.a = res;
    }

    fn or(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let a = self.rf.a;
        let b = if opcode == 0xF6 {
            self.fetch_u8()
        } else {
            self.read_r8(reg)
        };

        let res = a | b;
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(false);
        self.rf.a = res;
    }

    fn cp(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let a = self.rf.a;
        let b = if opcode == 0xFE {
            self.fetch_u8()
        } else {
            self.read_r8(reg)
        };

        let res = a.wrapping_sub(b);
        self.rf.write_z(res == 0);
        self.rf.write_n(true);
        self.rf.write_h((a & 0xF) < (b & 0xF));
        self.rf.write_c(a < b);
    }

    // ough
    fn daa(&mut self) {
        let mut res = self.rf.a;
        let mut offset = 0;
        offset |= 0x60 * (self.rf.read_c() as u8); // branchless gaming
        offset |= 0x06 * (self.rf.read_h() as u8);
        if self.rf.read_n() {
            res = res.wrapping_sub(offset);
        } else {
            offset |= 0x06 * ((res & 0xF > 0x9) as u8);
            offset |= 0x60 * ((res > 0x99) as u8);
            res = res.wrapping_add(offset);
        }

        self.rf.write_z(res == 0);
        self.rf.write_h(false);
        self.rf.write_c(offset >= 0x60);
        self.rf.a = res;
    }

    fn scf(&mut self) {
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(true);
    }

    fn cpl(&mut self) {
        self.rf.write_n(true);
        self.rf.write_h(true);
        self.rf.a = !self.rf.a;
    }

    fn ccf(&mut self) {
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(!self.rf.read_c());
    }

    fn add_sp_imm(&mut self) {
        let a = self.rf.sp;
        let b = self.fetch_u8();

        self.rf.write_z(false);
        self.rf.write_n(false);
        self.rf.write_h((a & 0xF) + ((b as u16) & 0xF) > 0xF);
        self.rf.write_c((a & 0xFF) + (b as u16) > 0xFF);

        self.rf.sp = a.wrapping_add_signed((b as i8) as i16);
    }

    // branches
    fn jr(&mut self, opcode: u8) -> usize {
        let offset = self.fetch_u8() as i8;
        let cond = match opcode {
            0x18 => true,
            0x28 => self.rf.read_z(),
            0x38 => self.rf.read_c(),
            0x20 => !self.rf.read_z(),
            0x30 => !self.rf.read_c(),
            _ => panic!("invalid opcode {:#04X} in jr", opcode),
        };
        if cond {
            self.rf.pc = self.rf.pc.wrapping_add_signed(offset as i16);
            INSTRUCTION_TIMINGS_BRANCH[opcode as usize]
        } else {
            INSTRUCTION_TIMINGS[opcode as usize]
        }
    }

    fn ret(&mut self, opcode: u8) -> usize {
        let cond = match opcode {
            0xC0 => !self.rf.read_z(),
            0xD0 => !self.rf.read_c(),
            0xC8 => self.rf.read_z(),
            0xD8 => self.rf.read_c(),
            0xC9 => true,
            0xD9 => {
                self.ime = true;
                true
            }
            _ => panic!("invalid opcode {:#04X} in ret", opcode),
        };

        if cond {
            self.rf.pc = self.pop_u16();
            INSTRUCTION_TIMINGS_BRANCH[opcode as usize]
        } else {
            INSTRUCTION_TIMINGS[opcode as usize]
        }
    }

    fn jp(&mut self, opcode: u8) -> usize {
        let addr = if opcode == 0xE9 {
            self.rf.read_hl()
        } else {
            self.fetch_u16()
        };

        let cond = match opcode {
            0xC2 => !self.rf.read_z(),
            0xD2 => !self.rf.read_c(),
            0xCA => self.rf.read_z(),
            0xDA => self.rf.read_c(),
            0xC3 => true,
            0xE9 => true,
            _ => panic!("invalid opcode {:#04X} in jp", opcode),
        };

        if cond {
            self.rf.pc = addr;
            INSTRUCTION_TIMINGS_BRANCH[opcode as usize]
        } else {
            INSTRUCTION_TIMINGS[opcode as usize]
        }
    }

    fn call(&mut self, opcode: u8) -> usize {
        let addr = self.fetch_u16();

        let cond = match opcode {
            0xC4 => !self.rf.read_z(),
            0xD4 => !self.rf.read_c(),
            0xCC => self.rf.read_z(),
            0xDC => self.rf.read_c(),
            0xCD => true,
            _ => panic!("invalid opcode {:#04X} in call", opcode),
        };

        if cond {
            self.push_u16(self.rf.pc);
            self.rf.pc = addr;
            INSTRUCTION_TIMINGS_BRANCH[opcode as usize]
        } else {
            INSTRUCTION_TIMINGS[opcode as usize]
        }
    }

    fn rst(&mut self, opcode: u8) {
        self.push_u16(self.rf.pc);
        self.rf.pc = (opcode & 0x38) as u16;
    }
}

// CB instructions
impl Cpu {
    fn rlc(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let val = self.read_r8(reg);

        let res = val.rotate_left(1);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x80 == 0x80);
        self.write_r8(reg, res);
    }

    fn rrc(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let val = self.read_r8(reg);

        let res = val.rotate_right(1);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x1 == 0x1);
        self.write_r8(reg, res);
    }

    fn rl(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let val = self.read_r8(reg);

        let res = (val << 1) | (self.rf.read_c() as u8);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x80 == 0x80);
        self.write_r8(reg, res);
    }

    fn rr(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let val = self.read_r8(reg);

        let res = (val >> 1) | ((self.rf.read_c() as u8) << 7);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x1 == 0x1);
        self.write_r8(reg, res);
    }

    fn sla(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let val = self.read_r8(reg);

        let res = val << 1;
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x80 == 0x80);
        self.write_r8(reg, res);
    }

    fn sra(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let val = self.read_r8(reg);

        let res = (val >> 1) | (val & 0x80);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x1 == 0x1);
        self.write_r8(reg, res);
    }

    fn swap(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let val = self.read_r8(reg);

        let res = ((val & 0xF0) >> 4) | ((val & 0x0F) << 4);
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(false);
        self.write_r8(reg, res);
    }

    fn srl(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let val = self.read_r8(reg);

        let res = val >> 1;
        self.rf.write_z(res == 0);
        self.rf.write_n(false);
        self.rf.write_h(false);
        self.rf.write_c(val & 0x1 == 0x1);
        self.write_r8(reg, res);
    }

    fn bit(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let idx = (opcode >> 3) & 7;
        let val = self.read_r8(reg);

        self.rf.write_z(val & (0x1 << idx) == 0);
        self.rf.write_n(false);
        self.rf.write_h(true);
    }

    fn res(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let idx = (opcode >> 3) & 7;
        let val = self.read_r8(reg);

        let res = val & !(0x1 << idx);
        self.write_r8(reg, res);
    }

    fn set(&mut self, opcode: u8) {
        let reg = opcode & 7;
        let idx = (opcode >> 3) & 7;
        let val = self.read_r8(reg);

        let res = val | (0x1 << idx);
        self.write_r8(reg, res);
    }
}
