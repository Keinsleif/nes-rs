use crate::apu::sounds::SquareNote;

const CPU_FREQ: f32 = 1789773.0;

pub struct SquareRegister {
    tone_volume: u8,
    sweep: u8,
    freq_lower: u8,
    freq_high_keyon: u8,
}

impl SquareRegister {
    pub fn new() -> Self {
        SquareRegister {
            tone_volume: 0x00,
            sweep: 0x00,
            freq_lower: 0x00,
            freq_high_keyon: 0x00,
        }
    }

    pub fn get_note(&self) -> SquareNote {
        SquareNote {
            freq: self.get_freq(),
            volume: self.get_volume(),
            duty: self.get_duty(),
        }
    }

    fn get_duty(&self) -> f32 {
        let flag = (self.tone_volume & 0b1100_0000) >> 6;
        match flag {
            0b00 => 0.125,
            0b01 => 0.25,
            0b10 => 0.50,
            0b11 => 0.75,
            _ => panic!("not possible")
        }
    }

    fn get_volume(&self) -> f32 {
        let vol = self.tone_volume & 0b0000_1111;
        vol as f32 / 0b0000_1111 as f32
    }

    fn get_freq(&self) -> f32 {
        let lower = self.freq_lower;
        let higher = self.freq_high_keyon & 0b0000_0111;
        CPU_FREQ / (16*(((higher as u16) << 8) | (lower as u16)+1)) as f32
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x4000 => {
                self.tone_volume = data;
            }
            0x4001 => {
                self.sweep = data;
            }
            0x4002 => {
                self.freq_lower = data;
            }
            0x4003 => {
                self.freq_high_keyon = data;
            }
            _ => panic!("not possible")
        }
    }
}
