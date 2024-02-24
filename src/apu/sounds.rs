mod noise;
mod pulse;

use std::sync::mpsc::Sender;

pub use noise::NoiseNote;
pub use noise::NoiseSound;
pub use pulse::PulseNote;
pub use pulse::PulseSound;

pub struct Transmitters {
    pub square1: Sender<PulseNote>,
    pub square2: Sender<PulseNote>,
    pub noise: Sender<NoiseNote>,
}

pub struct SoundManager {
    pub square1: PulseSound,
    pub square2: PulseSound,
    pub noise: NoiseSound,
}

impl SoundManager {
    pub fn new(sample_rate: f32) -> (Self, Transmitters) {
        let (square1_sound, square1_tx) = PulseSound::new(sample_rate);
        let (square2_sound, square2_tx) = PulseSound::new(sample_rate);
        let (noise_sound, noise_tx) = NoiseSound::new(sample_rate);
        (
            SoundManager {
                square1: square1_sound,
                square2: square2_sound,
                noise: noise_sound,
            },
            Transmitters {
                square1: square1_tx,
                square2: square2_tx,
                noise: noise_tx,
            },
        )
    }

    pub fn get_sound(&mut self) -> f32 {
        let mut sound: f32 = 0.0;
        sound += self.square1.step();
        sound += self.square2.step();
        sound += self.noise.step();
        sound
    }
}
