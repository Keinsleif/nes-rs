use crate::{apu::sounds::NoiseNote, cpu::CPU_FREQ};

pub struct NoiseRegister {
    is_length_counter_halt: bool,
    is_constant_volume: bool,
    volume: u8,
    is_long_period: bool,
    period: u8,
    length_counter_load: u8,
}

impl NoiseRegister {
    pub fn new() -> Self {
        NoiseRegister {
            is_length_counter_halt: true,
            is_constant_volume: false,
            volume: 0,
            is_long_period: true,
            period: 0,
            length_counter_load: 0,
        }
    }

    pub fn get_note(&self) -> NoiseNote {
        NoiseNote {
            volume: self.get_volume(),
            freq: self.get_freq(),
            is_long: self.is_long_period,
        }
    }

    fn get_volume(&self) -> f32 {
        (self.volume & 0b1111) as f32 / 0b1111 as f32
    }

    fn get_freq(&self) -> f32 {
        let t = match self.period {
            0 => 0x002,
            1 => 0x004,
            2 => 0x008,
            3 => 0x010,
            4 => 0x020,
            5 => 0x030,
            6 => 0x040,
            7 => 0x050,
            8 => 0x065,
            9 => 0x07f,
            10 => 0x0be,
            11 => 0x0fe,
            12 => 0x17d,
            13 => 0x1fc,
            14 => 0x3f9,
            15 => 0x7f2,
            _ => panic!("not possible")
        };
        CPU_FREQ as f32 / t as f32
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x400C => {
                self.is_length_counter_halt = data & 0b0010_0000 == 0;
                self.is_constant_volume = data & 0b0001_0000 != 0;
                self.volume = data & 0b0000_1111;
            },
            0x400D => {},
            0x400E => {
                self.is_long_period = data & 0b1000_0000 == 0;
                self.period = data & 0b0000_1111;
            },
            0x400F => {
                self.length_counter_load = (data & 0b1111_1000) >> 3;
            },
            _ => panic!("not possible")
        }
    }
}
