use crate::opcodes::OPCODE_MAP;
use crate::bus::Bus;

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
    Indirect,
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
    pub bus: Bus,
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

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
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }
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
            bus: Bus::new(),
        }
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read((0x0100 as u16) + self.stack_pointer as u16)
    }

    fn stack_push(&mut self, data: u8) {
        self.mem_write((0x0100 as u16) + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xff ) as u8;
        self.stack_push(high);
        self.stack_push(low);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop() as u16;
        let high = self.stack_pop() as u16;
        high << 8 | low
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.stack_pointer = 0xfd;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC)
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for i in 0..(program.len() as u16) {
            self.mem_write(0x0600 + i, program[i as usize])
        }
        self.mem_write_u16(0xFFFC, 0x0600);
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let pc_state = self.program_counter;

            let opcode = OPCODE_MAP.get(&code).unwrap();

            match opcode.name {
                "ADC" => {
                    self.adc(&opcode.mode);
                }
                "AND" => {
                    self.and(&opcode.mode)
                }
                "ASL" => {
                    self.asl(&opcode.mode);
                }
                "BIT" => {
                    self.bit(&opcode.mode);
                }
                "BCC" => {
                    self.branch(self.status & 0b0000_0001 == 0);
                }
                "BCS" => {
                    self.branch(self.status & 0b0000_0001 != 0);
                }
                "BEQ" => {
                    self.branch(self.status & 0b0000_0010 != 0);
                }
                "BMI" => {
                    self.branch(self.status & 0b1000_0000 != 0);
                }
                "BNE" => {
                    self.branch(self.status & 0b0000_0010 == 0);
                }
                "BPL" => {
                    self.branch(self.status & 0b1000_0000 == 0);
                }
                "BVC" => {
                    self.branch(self.status & 0b0100_0000 == 0);
                }
                "BVS" => {
                    self.branch(self.status & 0b0100_0000 != 0);
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
                "CMP" => {
                    self.cmp(&opcode.mode);
                }
                "CPX" => {
                    self.cpx(&opcode.mode);
                }
                "CPY" => {
                    self.cpy(&opcode.mode);
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
                "EOR" => {
                    self.eor(&opcode.mode);
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
                "JMP" => {
                    self.jmp(&opcode.mode);
                }
                "JSR" => {
                    self.jsr();
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
                "LSR" => {
                    self.lsr(&opcode.mode);
                }
                "NOP" => {}
                "ORA" => {
                    self.ora(&opcode.mode);
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
                "ROL" => {
                    self.rol(&opcode.mode);
                }
                "ROR" => {
                    self.ror(&opcode.mode);
                }
                "RTS" => {
                    self.rts();
                }
                "RTI" => {
                    self.rti();
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
            if pc_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }

            callback(self);
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

    fn asl(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::NoneAddressing => {
                if self.reg_a & 0b1000_0000 != 0 {
                    self.status |= 0b0000_0001;
                } else {
                    self.status &= 0b1111_1110;
                }
                self.reg_a <<= 1;
                self.update_zero_n_negative_flag(self.reg_a)
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let data = self.mem_read(addr);
                if data & 0b1000_0000 != 0 {
                    self.status |= 0b0000_0001;
                } else {
                    self.status &= 0b1111_1110;
                }
                let result = data << 1;
                self.mem_write(addr, result);
                self.update_zero_n_negative_flag(result);
            }
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        if self.reg_a & data == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
        if data & 0b0100_0000 != 0 {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
        if data & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump = self.mem_read(self.program_counter) as i8;
            let jump_addr = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
            self.program_counter = jump_addr;
        }
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

    fn compare(&mut self, mode: &AddressingMode, compare_with: u8) {
        let addr = self.get_operand_address(mode);
        let base_data = self.mem_read(addr);
        let data = (base_data as i8).wrapping_neg() as u8;

        let result = compare_with as u16 + data as u16;

        if result > 0xff {
            self.sec();
        } else {
            self.clc();
        }

        self.update_zero_n_negative_flag(result as u8);
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.reg_a);
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.reg_x);
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.reg_y);
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

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.reg_a ^= data;
        self.update_zero_n_negative_flag(self.reg_a);
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

    fn jmp(&mut self, mode: &AddressingMode) {
        self.program_counter = match mode {
            AddressingMode::NoneAddressing => {
                let mem_addr = self.mem_read_u16(self.program_counter);
                mem_addr
            }
            AddressingMode::Indirect => {
                let addr = self.mem_read_u16(self.program_counter);
                let indirect_addr = if addr & 0x00ff == 0x00ff {
                    let lo = self.mem_read(addr) as u16;
                    let hi = self.mem_read(addr & 0xff00) as u16;
                    hi << 8 | lo
                } else {
                    self.mem_read_u16(addr)
                };
                indirect_addr
            }
            _ => {
                panic!("Adressing mode {:?} is not supported", mode);
            }
        }
    }

    fn jsr(&mut self) {
        self.stack_push_u16(self.program_counter + 2 - 1 );
        self.program_counter = self.mem_read_u16(self.program_counter);
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

    fn lsr(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::NoneAddressing => {
                if self.reg_a & 0b0000_0001 != 0 {
                    self.status |= 0b0000_0001;
                } else {
                    self.status &= 0b1111_1110;
                }
                self.reg_a >>= 1;
                self.update_zero_n_negative_flag(self.reg_a)
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let data = self.mem_read(addr);
                if data & 0b0000_0001 != 0 {
                    self.status |= 0b0000_0001;
                } else {
                    self.status &= 0b1111_1110;
                }
                let result = data >> 1;
                self.mem_write(addr, result);
                self.update_zero_n_negative_flag(result);
            }
        }
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.reg_a |= data;
        self.update_zero_n_negative_flag(self.reg_a);
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
        self.status &= 0b1110_1111;
        self.status |= 0b0010_0000;
    }

    fn rol(&mut self, mode: &AddressingMode,) {
        let carry = self.status & 0b0000_0001 != 0;
        match mode {
            AddressingMode::NoneAddressing => {
                if self.reg_a & 0b1000_0000 != 0 {
                    self.status |= 0b0000_0001;
                } else {
                    self.status &= 0b1111_1110;
                }
                self.reg_a <<= 1;
                if carry {
                    self.reg_a |= 0b0000_0001;
                }
                self.update_zero_n_negative_flag(self.reg_a)
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let data = self.mem_read(addr);
                if data & 0b1000_0000 != 0 {
                    self.status |= 0b0000_0001;
                } else {
                    self.status &= 0b1111_1110;
                }
                let mut result = data << 1;
                if carry {
                    result |= 0b0000_0001;
                }
                self.mem_write(addr, result);
                self.update_zero_n_negative_flag(result);
            }
        }
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let carry = self.status & 0b0000_0001 != 0;
        match mode {
            AddressingMode::NoneAddressing => {
                if self.reg_a & 0b0000_0001 != 0 {
                    self.status |= 0b0000_0001;
                } else {
                    self.status &= 0b1111_1110;
                }
                self.reg_a >>= 1;
                if carry {
                    self.reg_a |= 0b1000_0000;
                }
                self.update_zero_n_negative_flag(self.reg_a)
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let data = self.mem_read(addr);
                if data & 0b0000_0001 != 0 {
                    self.status |= 0b0000_0001;
                } else {
                    self.status &= 0b1111_1110;
                }
                let mut result = data >> 1;
                if carry {
                    result |= 0b1000_0000;
                }
                self.mem_write(addr, result);
                self.update_zero_n_negative_flag(result);
            }
        }
    }

    fn rti(&mut self) {
        self.status = self.stack_pop();
        self.status &= 0b1110_1111;
        self.status |= 0b0010_0000;
        self.program_counter = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16() + 1;
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
            AddressingMode::Indirect => {
                panic!("Adressing Mode {:?} is not supported in this function", mode);
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
}
