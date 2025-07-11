use proc_bitfield::bitfield;

#[derive(Debug)]
pub struct Joypad {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    p1: P1,
    pub joypad_int: bool,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct P1(pub u8): Debug, FromStorage, IntoStorage, DerefStorage {
        pub buttons: bool @ 5,
        pub dpad: bool @ 4,
        pub start_down: bool @ 3,
        pub select_up: bool @ 2,
        pub b_left: bool @ 1,
        pub a_right: bool @ 0,
    }
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            select: false,
            start: false,
            p1: 0xFF.into(),
            joypad_int: false,
        }
    }

    pub fn write_u8(&mut self, val: u8) {
        self.p1 = val.into();
    }

    pub fn read_u8(&self) -> u8 {
        self.p1.into()
    }

    pub fn tick(&mut self) {
        if !self.p1.buttons() {
            if !self.start || !self.select || !self.b || !self.a {
                self.joypad_int = true;
            }
            self.p1.set_start_down(!self.start);
            self.p1.set_select_up(!self.select);
            self.p1.set_b_left(!self.b);
            self.p1.set_a_right(!self.a);
        } else if !self.p1.dpad() {
            if !self.down || !self.up || !self.left || !self.right {
                self.joypad_int = true;
            }
            self.p1.set_start_down(!self.down);
            self.p1.set_select_up(!self.up);
            self.p1.set_b_left(!self.left);
            self.p1.set_a_right(!self.right);
        } else {
            self.p1.set_start_down(true);
            self.p1.set_select_up(true);
            self.p1.set_b_left(true);
            self.p1.set_a_right(true);
        }
    }
}
