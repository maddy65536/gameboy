const TAC_VALS: [u64; 4] = [1024, 16, 64, 256];

#[derive(Debug)]
pub struct Timer {
    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    pub timer_int: bool,
    tima_state: u64,
    div_state: u64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            timer_int: false,
            tima_state: 0,
            div_state: 0,
        }
    }

    pub fn tick(&mut self, cycles: u64) {
        // update div
        self.div_state += cycles;
        while self.div_state >= 256 {
            self.div = self.div.wrapping_add(1);
            self.div_state -= 256;
        }

        // update tima
        if self.tac & 0b100 != 0 {
            let inc = TAC_VALS[(self.tac & 0b11) as usize];
            self.tima_state += cycles;

            while self.tima_state >= inc {
                self.tima_state -= inc;
                if self.tima == 0xFF {
                    self.tima = self.tma;
                    self.timer_int = true;
                } else {
                    self.tima = self.tima.wrapping_add(1);
                }
            }
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = val,
            0xFF06 => self.tma = val,
            0xFF07 => self.tac = val,
            _ => unreachable!(),
        }
    }

    pub fn reset_divider(&mut self) {
        self.div = 0;
    }
}
