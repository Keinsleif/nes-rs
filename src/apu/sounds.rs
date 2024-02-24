mod noise;
mod square;

use std::sync::mpsc::Sender;

pub use noise::NoiseNote;
pub use noise::NoiseSound;
pub use square::SquareNote;
pub use square::SquareSound;

pub struct Transmitters {
    pub square1: Sender<SquareNote>,
    pub square2: Sender<SquareNote>,
    pub noise: Sender<NoiseNote>,
}

pub struct SoundManager {
    pub square1: SquareSound,
    pub square2: SquareSound,
    pub noise: NoiseSound,
}

impl SoundManager {
    pub fn new(sample_rate: f32) -> (Self, Transmitters) {
        let (square1_sound, square1_tx) = SquareSound::new(sample_rate);
        let (square2_sound, square2_tx) = SquareSound::new(sample_rate);
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
