use std::sync::mpsc::{channel, Receiver, Sender};

pub struct SquareNote {
    pub freq: f32,
    pub volume: f32,
    pub duty: f32,
}

pub struct SquareSound {
    sample_rate: f32,
    phase: f32,
    pub note: SquareNote,
    pub rx: Receiver<SquareNote>,
}

impl SquareSound {
    pub fn new(sample_rate: f32, note: SquareNote) -> (Self, Sender<SquareNote>) {
        let (tx, rx) = channel::<SquareNote>();
        (
            SquareSound {
                sample_rate,
                phase: 0.0,
                note,
                rx,
            },
            tx,
        )
    }

    pub fn step(&mut self) -> f32{
        let output = if self.phase <= self.note.duty {
            self.note.volume
        } else {
            -self.note.volume
        };
        self.phase = (self.phase + self.note.freq / self.sample_rate) % 1.0;
        output
    }
}
