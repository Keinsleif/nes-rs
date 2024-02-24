pub mod apu;
pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod interrupt;
pub mod joypad;
pub mod opcodes;
pub mod ppu;
pub mod renderer;
pub mod trace;

use std::{collections::HashMap, env};

use apu::{
    sounds::{SoundManager, Transmitters},
    NesAPU,
};
use bus::Bus;
use cartridge::Rom;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, SizedSample, Stream,
};
use cpu::CPU;
use joypad::Joypad;
use ppu::NesPPU;
use renderer::frame::Frame;
// use trace::trace;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

#[macro_use]
extern crate bitflags;

fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys
        .window("NES Emulator", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    // init sound
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available.");

    let config = device.default_output_config().unwrap();

    let (stream, txs) = match config.sample_format() {
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

    stream.play().unwrap();

    // setup texture
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let args: Vec<String> = env::args().collect();

    let bytes = std::fs::read(&args[1]).unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let mut frame = Frame::new();

    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, joypad::JoypadButton::DOWN);
    key_map.insert(Keycode::Up, joypad::JoypadButton::UP);
    key_map.insert(Keycode::Left, joypad::JoypadButton::LEFT);
    key_map.insert(Keycode::Right, joypad::JoypadButton::RIGHT);
    key_map.insert(Keycode::LCtrl, joypad::JoypadButton::SELECT);
    key_map.insert(Keycode::LShift, joypad::JoypadButton::START);
    key_map.insert(Keycode::Z, joypad::JoypadButton::BUTTON_A);
    key_map.insert(Keycode::X, joypad::JoypadButton::BUTTON_B);

    let apu = NesAPU::new(txs);

    let bus = Bus::new(
        rom,
        move |ppu: &NesPPU, joypad: &mut Joypad| {
            renderer::render(ppu, &mut frame);
            texture.update(None, &frame.data, 256 * 3).unwrap();

            canvas.copy(&texture, None, None).unwrap();

            canvas.present();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => std::process::exit(0),
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                            joypad.set_button_pressed_status(*key, true);
                        }
                    }
                    Event::KeyUp { keycode, .. } => {
                        if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                            joypad.set_button_pressed_status(*key, false);
                        }
                    }
                    _ => { /* do nothing */ }
                }
            }
        },
        apu,
    );
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.run();
    // cpu.program_counter = 0xC000;

    // cpu.run_with_callback(|cpu| {
    //     println!("{}", trace::trace(cpu));
    // });
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> (Stream, Transmitters)
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let (mut sound_manager, txs) = SoundManager::new(sample_rate);
    let mut next_value = move || sound_manager.get_sound();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let value: T = T::from_sample(next_value() * 0.1);
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
            None,
        )
        .unwrap();
    (stream, txs)
}
