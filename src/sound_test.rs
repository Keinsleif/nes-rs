use std::{
    sync::mpsc::{channel, Receiver},
    time::Duration,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SizedSample, Stream
};

struct SquareNote {
    freq: f32,
    volume: f32,
    duty: f32,
}

struct SquareWave {
    sample_rate: f32,
    phase: f32,
    note: SquareNote,
    rx: Receiver<SquareNote>,
}

impl SquareWave {
    fn new(sample_rate: f32, note: SquareNote, rx: Receiver<SquareNote>) -> Self {
        SquareWave {
            sample_rate,
            phase: 0.0,
            note,
            rx,
        }
    }

    fn step(&mut self) -> f32{
        let output = if self.phase <= self.note.duty {
            self.note.volume
        } else {
            -self.note.volume
        };
        self.phase = (self.phase + self.note.freq / self.sample_rate) % 1.0;
        output
    }
}

fn main() {
    let (tx, rx) = channel::<SquareNote>();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available.");

    let config = device.default_output_config().unwrap();

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), rx),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), rx),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into(), rx),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), rx),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into(), rx),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), rx),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), rx),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), rx),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into(), rx),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), rx),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into(), rx),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), rx),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), rx),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), rx),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    };

    stream.play().unwrap();
    
    std::thread::sleep(std::time::Duration::from_millis(1000));

    tx.send(SquareNote {
        freq: 493.883,
        volume: 0.1,
        duty: 0.5,
    })
    .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, rx: Receiver<SquareNote>) -> Stream
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut wave = SquareWave::new(
        sample_rate,
        SquareNote {
            freq: 440.0,
            volume: 0.0,
            duty: 0.5,
        },
        rx,
    );
    let mut next_value = move || {
        let res = wave.rx.recv_timeout(Duration::ZERO);
        match res {
            Ok(note) => wave.note = note,
            Err(_) => {}
        }
        wave.step()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let value: T = T::from_sample(next_value());
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
            None,
        )
        .unwrap();
    stream
}
