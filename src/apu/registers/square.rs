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
}
