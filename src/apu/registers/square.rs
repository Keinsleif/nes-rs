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

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000 => {
                self.tone_volume = data;
            }
            0x0001 => {
                self.sweep = data;
            }
            0x0002 => {
                self.freq_lower = data;
            }
            0x0003 => {
                self.freq_high_keyon = data;
            }
            _ => panic!("not possible")
        }
    }
}
