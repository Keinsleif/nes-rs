mod registers;
pub mod sounds;

use std::sync::mpsc::Sender;

use registers::SquareRegister;
use registers::NoiseRegister;

use sounds::SquareNote;

use self::sounds::NoiseNote;

pub struct NesAPU {
    ch1: SquareRegister,
    ch2: SquareRegister,
    ch4: NoiseRegister,
    tx: Sender<SquareNote>,
    noise_tx: Sender<NoiseNote>
}

impl NesAPU {
    pub fn new(tx: Sender<SquareNote>, noise_tx: Sender<NoiseNote>) -> Self {
        NesAPU {
            ch1: SquareRegister::new(),
            ch2: SquareRegister::new(),
            ch4: NoiseRegister::new(),
            tx,
            noise_tx,
        }
    }

    pub fn write_square1(&mut self, addr: u16, data: u8) {
        self.ch1.write(addr, data);
        self.tx.send(self.ch1.get_note()).unwrap();
    }

    pub fn write_square2(&mut self, addr: u16, data: u8) {
        self.ch2.write(addr - 0x0004, data);
    }

    pub fn write_noise(&mut self, addr: u16, data: u8) {
        self.ch4.write(addr, data);
        self.noise_tx.send(self.ch4.get_note()).unwrap();
    }
}