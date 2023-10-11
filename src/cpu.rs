use crate::opcodes::OPCODE_MAP;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: 0,
            program_counter: 0,
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
        let high = self.mem_read(pos + 1) as u16;
        (high << 8) | low
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xFF) as u8;
        self.mem_write(pos, low);
        self.mem_write(pos + 1, high)
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC)
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let opcode = OPCODE_MAP.get(&code).unwrap();

            match opcode.name {
                "AND" => {
                    self.and(&opcode.mode)
                }
                "CLC" => {
                    self.clc();
                }
                "CLD" => {
                    self.cld();
                }
                "CLI" => {
                    self.cli();
                }
                "CLV" => {
                    self.clv();
                }
                "LDA" => {
                    self.lda(&opcode.mode);
                }
                "LDX" => {
                    self.ldx(&opcode.mode);
                }
                "LDY" => {
                    self.ldy(&opcode.mode);
                }
                "SEC" => {
                    self.sec();
                }
                "SED" => {
                    self.sed();
                }
                "SEI" => {
                    self.sei();
                }
                "TAX" => {
                    self.tax();
                }
                "INX" => {
                    self.inx()
                }
                "BRK" => {
                    return;
                }
                _ => todo!(""),
            }
            self.program_counter += (opcode.len - 1) as u16;
        }
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.reg_a = self.reg_a & self.mem_read(addr);
        self.update_zero_n_negative_flag(self.reg_a);
    }

    fn clc(&mut self) {
        self.status = self.status & 0b1111_1110
    }

    fn cld(&mut self) {
        self.status = self.status & 0b1111_0111;
    }

    fn cli(&mut self) {
        self.status = self.status & 0b1111_1011;
    }

    fn clv(&mut self) {
        self.status = self.status & 0b1101_1111;
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.reg_a = self.mem_read(addr);
        self.update_zero_n_negative_flag(self.reg_a)
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.reg_x = self.mem_read(addr);
        self.update_zero_n_negative_flag(self.reg_x)
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.reg_y = self.mem_read(addr);
        self.update_zero_n_negative_flag(self.reg_y)
    }

    fn sec(&mut self) {
        self.status = self.status | 0b0000_0001
    }

    fn sed(&mut self) {
        self.status = self.status | 0b0000_1000;
    }

    fn sei(&mut self) {
        self.status = self.status | 0b0000_0100;
    }

    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_zero_n_negative_flag(self.reg_x)
    }

    fn inx(&mut self) {
        self.reg_x = self.reg_x.wrapping_add(1);
        self.update_zero_n_negative_flag(self.reg_x)
    }

    fn update_zero_n_negative_flag(&mut self, result: u8) {
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

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.reg_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.reg_y) as u16
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_u16(self.program_counter);
                pos.wrapping_add(self.reg_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_u16(self.program_counter);
                pos.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let ptr = base.wrapping_add(self.reg_x);
                let low = self.mem_read(ptr as u16);
                let high = self.mem_read(ptr.wrapping_add(1) as u16);
                (high as u16) << 8 | (low as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
                let low = self.mem_read(base as u16);
                let high = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (high as u16) << 8 | (low as u16);
                deref_base.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("Addressing Mode {:?} is not supported", mode);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_ops(program: Vec<u8>) -> CPU {
        let mut cpu = CPU::new();
        cpu.load_and_run(program);
        cpu
    }

    #[test]
    fn test_0xa9_lda_immediate() {
        let cpu = run_ops(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa2_ldx_immediate() {
        let cpu = run_ops(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.reg_x, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa0_ldy_immediate() {
        let cpu = run_ops(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.reg_y, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let cpu = run_ops(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10)
    }

    #[test]
    fn test_0xaa_tax() {
        let cpu = run_ops(vec![0xa9, 0x10, 0xaa, 0x00]);
        assert_eq!(cpu.reg_x, 0x10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let cpu = run_ops(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let cpu = run_ops(vec![0xa2, 0xff, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_set_clear_flags() {
        let cpu = run_ops(vec![0x18, 0x38, 0x00]);
        assert!(cpu.status & 0b0000_0001 != 0);
        let cpu = run_ops(vec![0x38, 0x18, 0x00]);
        assert!(cpu.status & 0b0000_0001 == 0);
        let cpu = run_ops(vec![0xD8, 0xF8, 0x00]);
        assert!(cpu.status & 0b000_1000 != 0);
        let cpu = run_ops(vec![0xF8, 0xD8, 0x00]);
        assert!(cpu.status & 0b0000_1000 == 0);
        let cpu = run_ops(vec![0x58, 0x78, 0x00]);
        assert!(cpu.status & 0b0000_0100 != 0);
        let cpu = run_ops(vec![0x78, 0x58, 0x00]);
        assert!(cpu.status & 0b0000_0100 == 0);
    }

    #[test]
    fn test_and_immediate() {
        let cpu = run_ops(vec![0xa9, 0b0000_0001, 0x29, 0b0100_0011, 0x00]);
        assert_eq!(cpu.reg_a, 0b000_0001);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
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
        cpu.mem_write(0x20, 0x10);
        assert_eq!(cpu.memory[0x20 as usize], 0x10)
    }

    #[test]
    fn test_mem_read_u16() {
        let mut cpu: CPU = CPU::new();
        cpu.memory[0xFFFC as usize] = 0x10;
        let result = cpu.mem_read_u16(0xFFFC);
        assert_eq!(result, 0x10)
    }

    #[test]
    fn test_mem_write_u16() {
        let mut cpu: CPU = CPU::new();
        cpu.mem_write(0xFFFC, 0x10);
        assert_eq!(cpu.memory[0xFFFC as usize], 0x10)
    }
}
