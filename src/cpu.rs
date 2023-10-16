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
    pub stack_pointer: u8,
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
            stack_pointer: 0xfd,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn stack_push(&mut self, data: u8) {
        self.mem_write(0x0100 as u16 + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(0x0100 as u16 + self.stack_pointer as u16)
    }

    fn stack_push_u16(&mut self, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0b0000_1111) as u8;
        self.stack_push(high);
        self.stack_push(low);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop() as u16;
        let high = self.stack_pop() as u16;
        high << 8 | low
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
                "ADC" => {
                    self.adc(&opcode.mode);
                }
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
                "DEC" => {
                    self.dec(&opcode.mode);
                }
                "DEX" => {
                    self.dex()
                }
                "DEY" => {
                    self.dey()
                }
                "INC" => {
                    self.inc(&opcode.mode);
                }
                "INX" => {
                    self.inx()
                }
                "INY" => {
                    self.iny()
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
                "PHA" => {
                    self.pha();
                }
                "PHP" => {
                    self.php();
                }
                "PLA" => {
                    self.pla();
                }
                "PLP" => {
                    self.plp();
                }
                "SBC" => {
                    self.sbc(&opcode.mode);
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
                "STA" => {
                    self.sta(&opcode.mode);
                }
                "STX" => {
                    self.stx(&opcode.mode);
                }
                "STY" => {
                    self.sty(&opcode.mode)
                }
                "TAX" => {
                    self.tax();
                }
                "TAY" => {
                    self.tay();
                }
                "TSX" => {
                    self.tsx();
                }
                "TXA" => {
                    self.txa();
                }
                "TXS" => {
                    self.txs();
                }
                "TYA" => {
                    self.tya();
                }
                "BRK" => {
                    return;
                }
                _ => todo!(""),
            }
            self.program_counter += (opcode.len - 1) as u16;
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let carry = self.status & 0b0000_0001;

        let result = self.reg_a as u16 + data as u16 + carry as u16;

        if result > 0xff {
            self.sec();
        } else {
            self.clc();
        }

        if (self.reg_a ^ result as u8) & (data ^ result as u8) & 0x80 != 0 {
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & 0b1011_1111;
        }

        self.reg_a = result as u8;
        self.update_zero_n_negative_flag(self.reg_a);
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
        self.status = self.status & 0b1011_1111;
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let result = self.mem_read(addr).wrapping_sub(1);
        self.mem_write(addr, result);
        self.update_zero_n_negative_flag(result);
    }

    fn dex(&mut self) {
        self.reg_x = self.reg_x.wrapping_sub(1);
        self.update_zero_n_negative_flag(self.reg_x)
    }

    fn dey(&mut self) {
        self.reg_y = self.reg_y.wrapping_sub(1);
        self.update_zero_n_negative_flag(self.reg_y)
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let result = self.mem_read(addr).wrapping_add(1);
        self.mem_write(addr, result);
        self.update_zero_n_negative_flag(result);
    }

    fn inx(&mut self) {
        self.reg_x = self.reg_x.wrapping_add(1);
        self.update_zero_n_negative_flag(self.reg_x)
    }

    fn iny(&mut self) {
        self.reg_y = self.reg_y.wrapping_add(1);
        self.update_zero_n_negative_flag(self.reg_y);
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

    fn pha(&mut self) {
        self.stack_push(self.reg_a)
    }

    fn php(&mut self) {
        self.stack_push(self.status | 0b0011_0000)
    }

    fn pla(&mut self) {
        self.reg_a = self.stack_pop();
        self.update_zero_n_negative_flag(self.reg_a);
    }

    fn plp(&mut self) {
        self.status = self.stack_pop();
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let base_data = self.mem_read(addr);
        let data = (base_data as i8).wrapping_neg().wrapping_sub(1) as u8;

        let carry = self.status & 0b0000_0001;

        let result = self.reg_a as u16 + data as u16 + carry as u16;

        if result > 0xff {
            self.sec();
        } else {
            self.clc();
        }

        if (self.reg_a ^ result as u8) & (data ^ result as u8) & 0x80 != 0 {
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & 0b1011_1111;
        }

        self.reg_a = result as u8;
        self.update_zero_n_negative_flag(self.reg_a);
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

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.reg_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.reg_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.reg_y);
    }

    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_zero_n_negative_flag(self.reg_x)
    }

    fn tay(&mut self) {
        self.reg_y = self.reg_a;
        self.update_zero_n_negative_flag(self.reg_y);
    }

    fn tsx(&mut self) {
        self.reg_x = self.stack_pointer;
        self.update_zero_n_negative_flag(self.reg_x);
    }

    fn txa(&mut self) {
        self.reg_a = self.reg_x;
        self.update_zero_n_negative_flag(self.reg_a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.reg_x;
        self.update_zero_n_negative_flag(self.stack_pointer);
    }

    fn tya(&mut self) {
        self.reg_a = self.reg_y;
        self.update_zero_n_negative_flag(self.reg_a);
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
    fn test_0xa5_lda_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xb5_lda_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xa2, 0x08, 0xb5, 0x08, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xad_lda_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1020, 0x04);
        cpu.load_and_run(vec![0xad, 0x20, 0x10, 0x00]);
        assert_eq!(cpu.reg_a, 0x04);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xbd_lda_absolute_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1020, 0x04);
        cpu.load_and_run(vec![0xa2, 0x20, 0xbd, 0x00, 0x10, 0x00]);
        assert_eq!(cpu.reg_a, 0x04);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa1_lda_indirect_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x34, 0x34);
        cpu.mem_write(0x35, 0x12);
        cpu.mem_write_u16(0x1234, 0x04);
        cpu.load_and_run(vec![0xa2, 0x04, 0xa1, 0x30, 0x00]);
        assert_eq!(cpu.reg_a, 0x04);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xb1_lda_indirect_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x12, 0x30);
        cpu.mem_write(0x13, 0x12);
        cpu.mem_write_u16(0x1234, 0x03);
        cpu.load_and_run(vec![0xa0, 0x04, 0xb1, 0x12, 0x00]);
        assert_eq!(cpu.reg_a, 0x03);
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
    fn test_0x85_sta_zero_page() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x85, 0x12, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x05;
        cpu.run();
        let tmem = cpu.mem_read(0x12);
        assert_eq!(tmem, 0x05);
    }

    #[test]
    fn test_0x86_stx_zero_page() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x86, 0x12, 0x00]);
        cpu.reset();
        cpu.reg_x = 0x05;
        cpu.run();
        assert_eq!(cpu.mem_read(0x12), 0x05);
    }

    #[test]
    fn test_transfar_opcodes() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa,0xa8,0x9a,0x00]);
        cpu.reset();
        cpu.reg_a = 0x04;
        cpu.run();
        assert_eq!(0x04, cpu.reg_x);
        assert_eq!(0x04, cpu.reg_y);
        assert_eq!(0x04, cpu.stack_pointer);
        cpu.load(vec![0xba,0x8a,0x00]);
        cpu.reset();
        cpu.stack_pointer = 0x04;
        cpu.run();
        assert_eq!(0x04, cpu.reg_x);
        assert_eq!(0x04, cpu.reg_a);
        cpu.load(vec![0x98, 0x00]);
        cpu.reset();
        cpu.reg_y = 0x03;
        cpu.run();
        assert_eq!(0x03, cpu.reg_a);
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

    #[test]
    fn test_stack_push_pop() {
        let mut cpu = CPU::new();
        cpu.stack_push(0x10);
        let result = cpu.stack_pop();
        assert_eq!(result, 0x10)
    }

    #[test]
    fn test_0x48_pha() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0x15, 0x48, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.stack_pop(), 0x15);
    }

    #[test]
    fn test_0x08_php() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x08, 0x00]);
        assert_eq!(cpu.stack_pop(), 0b0011_0000);
    }

    #[test]
    fn test_0x68_pla() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x68, 0x00]);
        cpu.reset();
        cpu.stack_push(0x15);
        cpu.run();
        assert_eq!(cpu.reg_a, 0x15);
    }

    #[test]
    fn test_0x28_plp() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x28, 0x00]);
        cpu.reset();
        cpu.stack_push(0b1100_0100);
        cpu.run();
        assert_eq!(cpu.status, 0b1100_0100);
    }

    #[test]
    fn test_0xc6_dec_zero_page() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xc6, 0x14, 0x00]);
        cpu.reset();
        cpu.mem_write(0x14, 0x02);
        cpu.run();
        assert_eq!(cpu.mem_read(0x14), 0x01);
    }

    #[test]
    fn test_0xca_dex() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xca, 0x00]);
        cpu.reset();
        cpu.reg_x = 0x03;
        cpu.run();
        assert_eq!(cpu.reg_x, 0x02);
    }

    #[test]
    fn test_0xe6_inc_zero_page() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe6, 0x14, 0x00]);
        cpu.reset();
        cpu.mem_write(0x14, 0x13);
        cpu.run();
        assert_eq!(cpu.mem_read(0x14), 0x14);
    }

    #[test]
    fn test_0xe8_inx() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0x00]);
        cpu.reset();
        cpu.reg_x = 0x04;
        cpu.run();
        assert_eq!(cpu.reg_x, 0x05);
    }

    #[test]
    fn test_0x69_adc_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x18, 0x69, 0x10, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x50;
        cpu.run();
        assert_eq!(cpu.reg_a, 0x60);
    }

    #[test]
    fn test_0x69_adc_immediate_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x18, 0x69, 0x50, 0x00]);
        cpu.reset();
        cpu.reg_a = 0xd0;
        cpu.run();
        assert_eq!(cpu.reg_a, 0x20);
        assert!(cpu.status & 0b0000_0001 == 0b0000_0001);
    }

    #[test]
    fn test_0x69_adc_immediate_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x18, 0x69, 0x50, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x50;
        cpu.run();
        assert_eq!(cpu.reg_a, 0xa0);
        assert!(cpu.status & 0b0100_0000 == 0b0100_0000);
    }

    #[test]
    fn test_0x69_adc_immediate_carry_add() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x18, 0x69, 0x50, 0xa9, 0x10, 0x69, 0x50, 0x00]);
        cpu.reset();
        cpu.reg_a = 0xFE;
        cpu.run();
        assert_eq!(cpu.reg_a, 0x61);
        assert!(cpu.status & 0b0000_0001 != 0b0000_0001);
    }

    #[test]
    fn test_0xe9_sbc_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x38, 0xe9, 0xf0, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x50;
        cpu.run();
        assert_eq!(cpu.reg_a, 0x60);
    }

    #[test]
    fn test_0xe9_sbc_immediate_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x38, 0xe9, 0xf0, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x50;
        cpu.run();
        assert!(cpu.status & 0b0000_0001 == 0b0000_0000);
    }

    #[test]
    fn test_oxe9_sbc_immediate_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x38, 0xe9, 0xb0, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x50;
        cpu.run();
        assert!(cpu.status & 0b0100_0000 == 0b0100_0000);
    }

    #[test]
    fn test_oxe9_sbc_immediate_carry_sub() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x38, 0xe9, 0xf0, 0xa9, 0x10, 0xe9, 0x01, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x50;
        cpu.run();
        assert_eq!(cpu.reg_a, 0x0e);
    }
}
