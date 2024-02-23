mod registers;

use registers::SquareRegister;

pub struct NesAPU {
    ch1: SquareRegister,
    ch2: SquareRegister,
}

impl NesAPU {
    pub fn new() -> Self {
        NesAPU {
            ch1: SquareRegister::new(),
            ch2: SquareRegister::new(),
        }
    }

    pub fn write_square1(&mut self, addr: u16, data: u8) {
        self.ch1.write(addr - 0x4000, data);
    }

    pub fn write_square2(&mut self, addr: u16, data: u8) {
        self.ch2.write(addr - 0x4004, data);
    }
}