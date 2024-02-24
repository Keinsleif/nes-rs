use std::{sync::mpsc::{channel, Receiver, Sender}, time::Duration};

pub struct PulseNote {
    pub freq: f32,
    pub volume: f32,
    pub duty: f32,
}

impl PulseNote {
    pub fn new() -> Self {
        PulseNote {
            freq: 0.0,
            volume: 0.0,
            duty: 0.0,
        }
    }
}

pub struct PulseSound {
    sample_rate: f32,
    phase: f32,
    pub note: PulseNote,
    pub rx: Receiver<PulseNote>,
}

impl PulseSound {
    pub fn new(sample_rate: f32) -> (Self, Sender<PulseNote>) {
        let (tx, rx) = channel::<PulseNote>();
        (
            PulseSound {
                sample_rate,
                phase: 0.0,
                note: PulseNote::new(),
                rx,
            },
            tx,
        )
    }

    pub fn step(&mut self) -> f32{
        let res = self.rx.recv_timeout(Duration::ZERO);
        match res {
            Ok(note) => self.note = note,
            _ => {}
        }
        let output = if self.phase <= self.note.duty {
            self.note.volume
        } else {
            -self.note.volume
        };
        self.phase = (self.phase + self.note.freq / self.sample_rate) % 1.0;
        output
    }
}
