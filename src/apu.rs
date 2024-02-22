use self::registers::SquareRegister;

mod registers;

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
}