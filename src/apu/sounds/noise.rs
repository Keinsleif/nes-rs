use std::{sync::mpsc::{channel, Receiver, Sender}, time::Duration};

pub struct NoiseNote {
    pub freq: f32,
    pub volume: f32,
    pub is_long: bool,
}

impl NoiseNote {
    pub fn new() -> Self {
        NoiseNote {
            freq: 0.0,
            volume: 0.0,
            is_long: true,
        }
    }
}

pub struct NoiseRandom {
    is_long: bool,
    rand_reg: u16,
}

impl NoiseRandom {
    pub fn new(is_long: bool) -> Self {
        NoiseRandom {
            is_long,
            rand_reg: 0x01,
        }
    }

    pub fn next(&mut self) -> bool {
        let bit = if self.is_long {
            (self.rand_reg & 0x0001) ^ ((self.rand_reg >> 1) & 0x0001)
        } else {
            (self.rand_reg & 0x0001) ^ ((self.rand_reg >> 6) & 0x0001)
        };
        self.rand_reg >>= 1;
        self.rand_reg = (self.rand_reg & 0b11_1111_1111_1111) | bit << 14;
        self.rand_reg & 0x0001 != 0
    }
}

pub struct NoiseSound {
    sample_rate: f32,
    phase: f32,
    is_on: bool,
    long_rnd: NoiseRandom,
    short_rnd: NoiseRandom,
    pub note: NoiseNote,
    pub rx: Receiver<NoiseNote>,
}

impl NoiseSound {
    pub fn new(sample_rate: f32) -> (Self, Sender<NoiseNote>) {
        let (tx, rx) = channel::<NoiseNote>();
        (
            NoiseSound {
                sample_rate,
                phase: 0.0,
                is_on: false,
                long_rnd: NoiseRandom::new(true),
                short_rnd: NoiseRandom::new(false),
                note: NoiseNote::new(),
                rx,
            },
            tx,
        )
    }

    pub fn step(&mut self) -> f32 {
        let res = self.rx.recv_timeout(Duration::ZERO);
        match res {
            Ok(note) => self.note = note,
            _ => {}
        }
        let mut output = 0.0;
        if self.is_on {
            output = self.note.volume;
        }
        if (self.phase + self.note.freq / self.sample_rate) > 1.0 {
            self.is_on = if self.note.is_long {
                self.long_rnd.next()
            } else {
                self.short_rnd.next()
            };
        }
        self.phase = (self.phase + self.note.freq / self.sample_rate) % 1.0;
        output
    }
}