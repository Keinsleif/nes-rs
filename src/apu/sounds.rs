mod noise;
mod triangle;
mod pulse;

use std::sync::mpsc::Sender;

pub use noise::NoiseNote;
pub use noise::NoiseSound;
pub use triangle::TriangleNote;
pub use triangle::TriangleSound;
pub use pulse::PulseNote;
pub use pulse::PulseSound;

pub struct Transmitters {
    pub square1: Sender<PulseNote>,
    pub square2: Sender<PulseNote>,
    pub triangle: Sender<TriangleNote>,
    pub noise: Sender<NoiseNote>,
}

pub struct SoundManager {
    pub square1: PulseSound,
    pub square2: PulseSound,
    pub triangle: TriangleSound,
    pub noise: NoiseSound,
}

impl SoundManager {
    pub fn new(sample_rate: f32) -> (Self, Transmitters) {
        let (square1_sound, square1_tx) = PulseSound::new(sample_rate);
        let (square2_sound, square2_tx) = PulseSound::new(sample_rate);
        let (triangle_sound, triangle_tx) = TriangleSound::new(sample_rate);
        let (noise_sound, noise_tx) = NoiseSound::new(sample_rate);
        (
            SoundManager {
                square1: square1_sound,
                square2: square2_sound,
                triangle: triangle_sound,
                noise: noise_sound,
            },
            Transmitters {
                square1: square1_tx,
                square2: square2_tx,
                triangle: triangle_tx,
                noise: noise_tx,
            },
        )
    }

    pub fn get_sound(&mut self) -> f32 {
        let mut sound: f32 = 0.0;
        sound += self.square1.step();
        sound += self.square2.step();
        sound += self.triangle.step();
        sound += self.noise.step();
        sound
    }
}
