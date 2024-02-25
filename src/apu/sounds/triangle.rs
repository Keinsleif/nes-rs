use std::{sync::mpsc::{channel, Receiver, Sender}, time::Duration};

pub struct TriangleNote {
    pub freq: f32,
}

impl TriangleNote {
    pub fn new() -> Self {
        TriangleNote {
            freq: 0.0,
        }
    }
}

pub struct TriangleSound {
    sample_rate: f32,
    phase: f32,
    pub note: TriangleNote,
    pub rx: Receiver<TriangleNote>,
}

impl TriangleSound {
    // const OUTPUT: [u8; 32] = [
    //     15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    //     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    // ];
    pub fn new(sample_rate: f32) -> (Self, Sender<TriangleNote>) {
        let (tx, rx) = channel::<TriangleNote>();
        (
            TriangleSound {
                sample_rate,
                phase: 0.0,
                note: TriangleNote::new(),
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
        let output = if self.phase <= 0.25 {
            self.phase
        } else if self.phase >= 0.25 && self.phase <= 0.75 {
            0.5 - self.phase
        } else {
            self.phase - 1.0
        };
        self.phase = (self.phase + self.note.freq / self.sample_rate) % 1.0;
        output * 4.0
    }
}
