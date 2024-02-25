use crate::apu::sounds::TriangleNote;
use crate::cpu::CPU_FREQ;

pub struct TriangleRegister {
    is_length_counter_halt: bool,
    counter_reload_value: u8,

    timer: u16,
    length_counter_load: u8, 
}

impl TriangleRegister {
    pub fn new() -> Self {
        TriangleRegister {
            is_length_counter_halt: true,
            counter_reload_value: 0,

            timer: 0,
            length_counter_load: 0,
        }
    }

    pub fn get_note(&self) -> TriangleNote {
        TriangleNote {
            freq: self.get_freq(),
        }
    }

    fn get_freq(&self) -> f32 {
        CPU_FREQ / (32*(self.timer+1)) as f32
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x4008 => {
                self.is_length_counter_halt = data & 0b1000_0000 == 0;
                self.counter_reload_value = data & 0b0111_1111;
            }
            0x4009 => {}
            0x400A => {
                self.timer = (self.timer & 0b111_0000_0000) | data as u16;
            }
            0x400B => {
                self.length_counter_load = (data & 0b1111_1000) >> 3;
                self.timer = (self.timer & 0b000_1111_1111) | ((data & 0b0000_0111) as u16) << 8;
            }
            _ => panic!("not possible")
        }
    }
}
