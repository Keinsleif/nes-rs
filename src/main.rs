pub mod cpu;
pub mod bus;
pub mod opcodes;
pub mod cartridge;
pub mod trace;
pub mod ppu;
pub mod interrupt;
pub mod renderer;
pub mod joypad;

use std::{collections::HashMap, env};

use bus::Bus;
use cartridge::Rom;
use cpu::CPU;
use joypad::Joypad;
use ppu::NesPPU;
use renderer::frame::Frame;
// use trace::trace;
use sdl2::{pixels::PixelFormatEnum, event::Event, keyboard::Keycode};

#[macro_use]
extern crate bitflags;

fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys
    .window("NES Emulator", (256.0 * 3.0) as u32 , (240.0 * 3.0) as u32)
    .position_centered()
    .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    // setup texture
    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormatEnum::RGB24, 256, 240).unwrap();

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

    let bus = Bus::new(rom, move |ppu: &NesPPU, joypad: &mut Joypad| {
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
    });
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.run();
    // cpu.program_counter = 0xC000;

    // let mut screen_state = [0 as u8; 32 * 3 * 32];
    // let mut rng = rand::thread_rng();

    // cpu.run_with_callback(move |cpu| {
    //     println!("{}", trace(cpu));
    //     // handle_user_input(cpu, &mut event_pump);
    //     // cpu.mem_write(0xfe, rng.gen_range(1, 16));

    //     // if read_screen_state(cpu, &mut screen_state) {
    //     //     texture.update(None, &screen_state, 32*3).unwrap();
    //     //     canvas.copy(&texture, None, None).unwrap();
    //     //     canvas.present();
    //     // }

    //     // ::std::thread::sleep(std::time::Duration::new(0, 70_000));
    // });
}
