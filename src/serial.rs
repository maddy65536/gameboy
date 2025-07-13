use proc_bitfield::bitfield;

use crate::gameboy::CLOCK_SPEED;

const UPDATE_FREQ: usize = CLOCK_SPEED / 8192;

// this is just a stub, maybe i'll fill it out eventually
#[derive(Debug)]
pub struct Serial {
    pub sb: u8,
    pub sc: Sc,
    pub serial_int: bool,
    cycles: usize,
    bit_counter: u8,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Sc(pub u8): Debug, FromStorage, IntoStorage, DerefStorage {
        pub transfer_enable: bool @ 7,
        pub clock_speed: bool @ 1, // CGB only
        pub clock_select: bool @ 0,
    }
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0.into(),
            serial_int: false,
            cycles: 0,
            bit_counter: 0,
        }
    }

    pub fn tick(&mut self, cycles: usize) {
        if !self.sc.clock_select() || !self.sc.transfer_enable() {
            return;
        }

        self.cycles += cycles;
        if self.cycles >= UPDATE_FREQ {
            self.cycles -= UPDATE_FREQ;
            self.bit_counter += 1;
            self.sb = (self.sb << 1) | 1;

            if self.bit_counter >= 8 {
                self.bit_counter = 0;
                self.sc.set_transfer_enable(false);
                self.serial_int = true;
            }
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0xFF01 => self.sb,
            0xFF02 => self.sc.into(),
            _ => unimplemented!(),
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF01 => self.sb = val,
            0xFF02 => self.sc = val.into(),
            _ => unimplemented!(),
        }
    }
}
