use std::{
    sync::mpsc::{channel, Receiver},
    time::Duration,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, OutputCallbackInfo, Sample, SizedSample, StreamConfig,
};

struct SquareNote {
    hz: f32,
    volume: f32,
    duty: f32,
}

struct SquareWave {
    freq: f32,
    phase: f32,
    note: SquareNote,
    rx: Receiver<SquareNote>,
}

impl SquareWave {
    fn new(freq: f32, note: SquareNote, rx: Receiver<SquareNote>) -> Self {
        SquareWave {
            freq,
            phase: 0.0,
            note,
            rx,
        }
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available.");

    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into()),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into()),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    };
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig)
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let (tx, rx) = channel::<SquareNote>();

    // Produce a sinusoid of maximum amplitude.
    let mut wave = SquareWave::new(
        sample_rate,
        SquareNote {
            hz: 440.0,
            volume: 0.1,
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
        let output = if wave.phase <= wave.note.duty {
            wave.note.volume
        } else {
            -wave.note.volume
        };
        wave.phase = (wave.phase + wave.note.hz / wave.freq) % 1.0;
        output
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )
        .unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));

    tx.send(SquareNote {
        hz: 493.883,
        volume: 0.1,
        duty: 0.5,
    })
    .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
