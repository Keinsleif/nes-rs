mod registers;
pub mod sounds;

use registers::PulseRegister;
use registers::NoiseRegister;
use self::sounds::Transmitters;

pub struct NesAPU {
    square1: PulseRegister,
    square2: PulseRegister,
    noise: NoiseRegister,
    txs: Transmitters,
}

impl NesAPU {
    pub fn new(txs: Transmitters) -> Self {
        NesAPU {
            square1: PulseRegister::new(),
            square2: PulseRegister::new(),
            noise: NoiseRegister::new(),
            txs
        }
    }

    pub fn write_square1(&mut self, addr: u16, data: u8) {
        self.square1.write(addr, data);
        self.txs.square1.send(self.square1.get_note()).unwrap();
    }

    pub fn write_square2(&mut self, addr: u16, data: u8) {
        self.square2.write(addr - 0x0004, data);
        self.txs.square2.send(self.square2.get_note()).unwrap();
    }

    pub fn write_noise(&mut self, addr: u16, data: u8) {
        self.noise.write(addr, data);
        self.txs.noise.send(self.noise.get_note()).unwrap();
    }
}