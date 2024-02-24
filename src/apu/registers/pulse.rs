use crate::apu::sounds::PulseNote;
use crate::cpu::CPU_FREQ;

pub struct PulseRegister {
    duty: u8,
    is_length_counter_halt: bool,
    is_constant_volume: bool,
    volume: u8,

    is_sweep_enable: bool,
    sweep_timer_count: u8,
    is_sweep_negate: bool,
    sweep_shift_count: u8,

    timer: u16,
    length_counter_load: u8, 
}

impl PulseRegister {
    pub fn new() -> Self {
        PulseRegister {
            duty: 0,
            is_length_counter_halt: true,
            is_constant_volume: false,
            volume: 0,
        
            is_sweep_enable: false,
            sweep_timer_count: 0,
            is_sweep_negate: true,
            sweep_shift_count: 0,
        
            timer: 0,
            length_counter_load: 0,
        }
    }

    pub fn get_note(&self) -> PulseNote {
        PulseNote {
            freq: self.get_freq(),
            volume: self.get_volume(),
            duty: self.get_duty(),
        }
    }

    fn get_duty(&self) -> f32 {
        match self.duty {
            0b00 => 0.125,
            0b01 => 0.25,
            0b10 => 0.50,
            0b11 => 0.75,
            _ => panic!("not possible")
        }
    }

    fn get_volume(&self) -> f32 {
        let vol = self.volume & 0b0000_1111;
        vol as f32 / 0b0000_1111 as f32
    }

    fn get_freq(&self) -> f32 {
        CPU_FREQ / (16*(self.timer+1)) as f32
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x4000 => {
                self.duty = (data & 0b1100_0000) >> 6;
                self.is_length_counter_halt = data & 0b0010_0000 == 0;
                self.is_constant_volume = data & 0b0001_0000 != 0;
                self.volume = data & 0b0000_1111;
            }
            0x4001 => {
                self.is_sweep_enable = data & 0b1000_0000 != 0;
                self.sweep_timer_count = (data & 0b0111_0000) >> 4;
                self.is_sweep_negate = data & 0b0000_1000 == 0;
                self.sweep_shift_count = data & 0b0000_0111;
            }
            0x4002 => {
                self.timer = (self.timer & 0b111_0000_0000) | data as u16;
            }
            0x4003 => {
                self.length_counter_load = (data & 0b1111_1000) >> 3;
                self.timer = (self.timer & 0b000_1111_1111) | ((data & 0b0000_0111) as u16) << 8;
            }
            _ => panic!("not possible")
        }
    }
}
