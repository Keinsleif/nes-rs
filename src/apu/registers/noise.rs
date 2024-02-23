use crate::{apu::sounds::NoiseNote, cpu::CPU_FREQ};

pub struct NoiseRegister {
    volume: u8,
    freq_cycle: u8,
    keyon_off: u8,
}

impl NoiseRegister {
    pub fn new() -> Self {
        NoiseRegister {
            volume: 0x00,
            freq_cycle: 0x00,
            keyon_off: 0x00,
        }
    }

    pub fn get_note(&self) -> NoiseNote {
        NoiseNote {
            volume: self.get_volume(),
            freq: self.get_freq(),
            is_long: self.is_long(),
        }
    }

    fn is_long(&self) -> bool {
        self.freq_cycle >> 7 != 0
    }

    fn get_volume(&self) -> f32 {
        (self.volume & 0b1111) as f32 / 0b1111 as f32
    }

    fn get_freq(&self) -> f32 {
        let t = match self.freq_cycle & 0b0000_1111 {
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
            0x400C => self.volume = data,
            0x400D => {},
            0x400E => self.freq_cycle = data,
            0x400F => self.keyon_off = data,
            _ => panic!("not possible")
        }
    }
}
