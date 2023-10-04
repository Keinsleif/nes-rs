pub struct CPU {
    pub reg_a: u8,
    pub status: u8,
    pub pc: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_a: 0,
            status: 0,
            pc: 0,
        }
    }

    fn lda(&mut self,value: u8) {
        self.reg_a = value;
        self.update_zero_n_negative_flag(self.reg_a)
    }

    fn update_zero_n_negative_flag(&mut self,result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;
        
        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                0xA9 => {
                    let param = program[self.pc as usize];
                    self.pc += 1;
                    self.lda(param)
                }
                0x00 => {
                    return;
                }
                _ => todo!("")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9,0x05,0x00]);
        assert_eq!(cpu.reg_a,0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9,0x00,0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10)
    }
}
