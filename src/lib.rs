pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub status: u8,
    pub pc: u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_a: 0,
            reg_x: 0,
            status: 0,
            pc: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let low = self.mem_read(pos) as u16;
        let high = self.mem_read(pos+1) as u16;
        (high << 8) | low
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xFF) as u8;
        self.mem_write(pos, low);
        self.mem_write(pos+1, high)
    }

    pub fn load_and_run(&mut self,program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.status = 0;

        self.pc = self.mem_read_u16(0xFFFC)
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.mem_read(self.pc);
            self.pc += 1;

            match opcode {
                _ => todo!("")
            }
        }
    }

    fn lda(&mut self,value: u8) {
        self.reg_a = value;
        self.update_zero_n_negative_flag(self.reg_a)
    }

    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_zero_n_negative_flag(self.reg_x)
    }

    fn inx(&mut self) {
        if self.reg_x == 0xff {
            self.reg_x = 0;
        } else {
            self.reg_x += 1;
        }
        self.update_zero_n_negative_flag(self.reg_x)
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
                    self.lda(param);
                }
                0xAA => {
                    self.tax();
                }
                0xE8 => {
                    self.inx();
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

    #[test]
    fn test_0xaa_tax() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x10;
        cpu.interpret(vec![0xaa,0x00]);
        assert_eq!(cpu.reg_x, 0x10)
    }

    #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.reg_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_mem_read() {
        let mut cpu = CPU::new();
        cpu.memory[0x20 as usize] = 0x10;
        let result = cpu.mem_read(0x20);
        assert_eq!(result, 0x10)
    }

    #[test]
    fn test_mem_write() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x20,0x10);
        assert_eq!(cpu.memory[0x20 as usize],0x10)
    }

    #[test]
    fn test_mem_read_u16() {
        let mut cpu: CPU = CPU::new();
        cpu.memory[0xFFFC as usize] = 0x10;
        let result = cpu.mem_read_u16(0xFFFC);
        assert_eq!(result,0x10)
    }

    #[test]
    fn test_mem_write_u16() {
        let mut cpu: CPU = CPU::new();
        cpu.mem_write(0xFFFC, 0x10);
        assert_eq!(cpu.memory[0xFFFC as usize], 0x10)
    }
}
