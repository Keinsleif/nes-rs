mod registers;
pub mod sounds;

use std::sync::mpsc::Sender;

use registers::SquareRegister;

use self::sounds::SquareNote;

pub struct NesAPU {
    ch1: SquareRegister,
    ch2: SquareRegister,
    tx: Sender<SquareNote>,
}

impl NesAPU {
    pub fn new(tx: Sender<SquareNote>) -> Self {
        NesAPU {
            ch1: SquareRegister::new(),
            ch2: SquareRegister::new(),
            tx,
        }
    }

    pub fn write_square1(&mut self, addr: u16, data: u8) {
        self.ch1.write(addr - 0x4000, data);
        self.tx.send(self.ch1.get_note()).unwrap();
    }

    pub fn write_square2(&mut self, addr: u16, data: u8) {
        self.ch2.write(addr - 0x4004, data);
    }
}