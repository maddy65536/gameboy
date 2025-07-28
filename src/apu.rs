use rodio::{OutputStream, Sink, buffer::SamplesBuffer};

const AUDIO_DEBUG: bool = false;

const WAVE_DUTY_CYCLES: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
    [1, 0, 0, 0, 0, 0, 0, 1], // 25%
    [1, 0, 0, 0, 0, 1, 1, 1], // 50%
    [0, 1, 1, 1, 1, 1, 1, 0], // 75%
];

const AUDIO_SAMPLE_RATE: u32 = 44100;
const SAMPLE_FREQ: u64 = crate::gameboy::CLOCK_SPEED / (AUDIO_SAMPLE_RATE as u64); // how often to output a sample
const SAMPLE_BUF_SIZE: usize = 4096; // how many samples to output at once, arbitrary

fn dac(digital: u8) -> f32 {
    (((digital as f32) / 15.0) * 2.0) - 1.0
}

pub struct Apu {
    ch1: Channel1,
    ch2: Channel2,
    ch3: Channel3,
    ch4: Channel4,

    nr50: u8, // master volume & VIN panning
    nr51: u8, // sound panning
    nr52: u8, // sound on/off
    wave_ram: [u8; 0x10],

    sink: Sink,
    _stream: OutputStream, // stream needs to be kept alive
    cycles: u64,

    sample_buffer: Vec<f32>,

    div_apu: u8,
    div_bit: bool,
    capacitor: f32,
}

impl Apu {
    pub fn new() -> Self {
        let stream = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(stream.mixer());

        Self {
            ch1: Channel1::default(),
            ch2: Channel2::default(),
            ch3: Channel3::default(),
            ch4: Channel4::default(),
            nr50: 0,
            nr51: 0,
            nr52: 0,
            wave_ram: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            sink,
            _stream: stream,
            cycles: 0,
            sample_buffer: Vec::with_capacity(SAMPLE_BUF_SIZE),
            div_apu: 0,
            div_bit: false,
            capacitor: 0.0,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0xFF10 => self.ch1.nr10,
            0xFF11 => self.ch1.nr11 | 0x3F, // set all write only bits to 1
            0xFF12 => self.ch1.nr12,
            0xFF13 => self.ch1.nr13 | 0xFF,
            0xFF14 => self.ch1.nr14 | 0xBF,
            0xFF16 => self.ch2.nr21,
            0xFF17 => self.ch2.nr22,
            0xFF18 => self.ch2.nr23,
            0xFF19 => self.ch2.nr24,
            0xFF1A => self.ch3.nr30,
            0xFF1B => self.ch3.nr31,
            0xFF1C => self.ch3.nr32,
            0xFF1D => self.ch3.nr33,
            0xFF1E => self.ch3.nr34,
            0xFF20 => self.ch4.nr41,
            0xFF21 => self.ch4.nr42,
            0xFF22 => self.ch4.nr43,
            0xFF23 => self.ch4.nr44,
            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => self.nr52,
            0xFF30..=0xFF3F => self.wave_ram[(addr as usize) - 0xFF30],
            _ => panic!("invalid read from APU at address {addr:#06x}"),
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        // mask write only channels
        match addr {
            0xFF10 => self.ch1.nr10 = val,
            0xFF11 => self.ch1.nr11 = val,
            0xFF12 => self.ch1.nr12 = val,
            0xFF13 => self.ch1.nr13 = val,
            0xFF14 => self.ch1.nr14 = val,
            0xFF16 => self.ch2.nr21 = val,
            0xFF17 => self.ch2.nr22 = val,
            0xFF18 => self.ch2.nr23 = val,
            0xFF19 => self.ch2.nr24 = val,
            0xFF1A => self.ch3.nr30 = val,
            0xFF1B => self.ch3.nr31 = val,
            0xFF1C => self.ch3.nr32 = val,
            0xFF1D => self.ch3.nr33 = val,
            0xFF1E => self.ch3.nr34 = val,
            0xFF20 => self.ch4.nr41 = val,
            0xFF21 => self.ch4.nr42 = val,
            0xFF22 => self.ch4.nr43 = val,
            0xFF23 => self.ch4.nr44 = val,
            0xFF24 => self.nr50 = val,
            0xFF25 => self.nr51 = val,
            0xFF26 => self.nr52 = (val & 0x80) | (self.nr52 & !0x80), // don't set read only
            0xFF30..=0xFF3F => self.wave_ram[(addr as usize) - 0xFF30] = val,
            _ => panic!("invalid write to APU at address {addr:#06x}"),
        }
    }

    pub fn tick(&mut self, cycles: u64, div: u8) {
        // simpler to emulate this one cycle at a time
        for _ in 0..cycles {
            // update div_apu
            if self.div_bit && (div & 0x08) == 0 {
                self.div_apu = self.div_apu.wrapping_add(1);
            }
            self.div_bit = (div & 0x08) != 0;

            // tick channels, get samples
            let ch1 = self.ch1.tick(self.div_apu);
            let ch2 = 0.0;
            let ch3 = 0.0;
            let ch4 = 0.0;

            // TODO: handle mixer
            let mut sample = ch1 + ch2 + ch3 + ch4;
            sample /= 4.0;
            sample = self.high_pass(sample);

            self.cycles += 1;
            if self.cycles > SAMPLE_FREQ {
                self.cycles -= SAMPLE_FREQ;
                self.sample_buffer.push(sample);
                self.sample_buffer.push(sample);
            }
            if self.sample_buffer.len() >= SAMPLE_BUF_SIZE && !AUDIO_DEBUG {
                self.sink.append(SamplesBuffer::new(
                    2,
                    AUDIO_SAMPLE_RATE,
                    self.sample_buffer.clone(),
                ));
                self.sample_buffer.clear();
            }
        }
    }

    fn high_pass(&mut self, sample: f32) -> f32 {
        let res = sample - self.capacitor;
        self.capacitor = sample - res * 0.999958;
        res
    }
}

impl Drop for Apu {
    fn drop(&mut self) {
        if AUDIO_DEBUG {
            std::fs::write(
                "audio_log.pcm",
                self.sample_buffer
                    .iter()
                    .flat_map(|s| s.to_le_bytes())
                    .collect::<Vec<u8>>(),
            )
            .unwrap();
        }
    }
}

#[derive(Debug, Default)]
struct Channel1 {
    nr10: u8, // sweep
    nr11: u8, // length timer & duty cycle
    nr12: u8, // volume & envelope
    nr13: u8, // period low
    nr14: u8, // period high & control

    cycles: u8,
    duty_step: usize,
    length_timer: u16,
    div_cache: u8,
    period_div: u16,
}

impl Channel1 {
    fn tick(&mut self, div_apu: u8) -> f32 {
        if self.nr14 & 0x80 != 0 {
            self.nr14 &= 0x7F; // disable trigger bit
            self.trigger();
        }

        // div has changed, update stuff
        if div_apu != self.div_cache {
            self.div_cache = div_apu;
            if div_apu % 2 == 0 {
                // length timer
            }
        }

        self.cycles += 1;
        if self.cycles >= 4 {
            self.cycles -= 4;
            self.period_div += 1;
            if self.period_div > 2047 {
                self.period_div = (self.nr13 as u16) | (((self.nr14 as u16) & 0x7) << 8);
                self.duty_step = (self.duty_step + 1) % 8;
            }
        }

        let res = WAVE_DUTY_CYCLES[((self.nr11 as usize) >> 6) & 0x03][self.duty_step];
        dac(res)
    }

    fn trigger(&mut self) {}
}

#[derive(Debug, Default)]
struct Channel2 {
    nr21: u8, // length timer & duty cycle
    nr22: u8, // volume & envelope
    nr23: u8, // period low
    nr24: u8, // period high & control
}

#[derive(Debug, Default)]
struct Channel3 {
    nr30: u8, // DAC enable
    nr31: u8, // length timer
    nr32: u8, // output level
    nr33: u8, // period low
    nr34: u8, // period high & control
}

#[derive(Debug, Default)]
struct Channel4 {
    nr41: u8, // length timer
    nr42: u8, // volume & envelope
    nr43: u8, // frequency & randomness
    nr44: u8, // control
}
