mod addressing_mode;
mod op_codes;

use addressing_mode::AddressingMode;

bitflags::bitflags! {
    #[derive(Clone, Debug)]
    pub struct StatusFlags: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE = 0b0000_1000;
        const BREAK = 0b0001_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

#[derive(Clone, Debug)]
pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,

    pub status: StatusFlags,
    /// Program Counter
    pub pc: u16,
    pub sp: u8,

    memory: [u8; 0xFFFF],
}

const STACK_OFFSET: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;
const PC_OFFSET: u16 = 0x8000;

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: StatusFlags::INTERRUPT_DISABLE,
            pc: 0,
            sp: STACK_RESET,
            memory: [0; 0xFFFF],
        }
    }

    fn pop_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.mem_read(self.sp as u16 + STACK_OFFSET)
    }

    fn push_stack(&mut self, value: u8) {
        self.mem_write(self.sp as u16 + STACK_OFFSET, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop_stack_u16(&mut self) -> u16 {
        let lo = self.pop_stack() as u16;
        let hi = self.pop_stack() as u16;
        (hi << 8) | lo
    }

    fn push_stack_u16(&mut self, value: u16) {
        let lo = value as u8;
        let hi = (value >> 8) as u8;
        self.push_stack(hi);
        self.push_stack(lo);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[(PC_OFFSET as usize) + i] = byte;
        }

        self.mem_write_u16(0xFFFC, PC_OFFSET);
    }

    /// When a cartridge was inserted into the NES, a reset signal was sent to the CPU. This
    /// function emulates that reset signal.
    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.status = StatusFlags::from_bits_truncate(0);

        self.pc = self.mem_read_u16(0xFFFC);
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        (hi << 8) | lo
    }

    pub fn mem_write_u16(&mut self, addr: u16, value: u16) {
        let lo = value as u8;
        let hi = (value >> 8) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }

    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        if value == 0 {
            self.status.insert(StatusFlags::ZERO);
        } else {
            self.status.remove(StatusFlags::ZERO);
        }

        if value & 0b1000_0000 != 0 {
            self.status.insert(StatusFlags::NEGATIVE);
        } else {
            self.status.remove(StatusFlags::NEGATIVE);
        }
    }

    pub fn update_carry(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.status.insert(StatusFlags::CARRY);
        } else {
            self.status.remove(StatusFlags::CARRY);
        }
    }

    /// The NES emulator has multiple different addressing modes to access memeory. This function
    /// takes in an addressing mode and returns the memory location for the byte to be read.
    fn get_op_addr(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,

            AddressingMode::ZeroPage => self.mem_read(self.pc) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.pc),

            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.pc);
                let addr = pos.wrapping_add(self.reg_x) as u16;
                addr
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.pc);
                let addr = pos.wrapping_add(self.reg_y) as u16;
                addr
            }

            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.pc);
                let addr = base.wrapping_add(self.reg_x as u16);
                addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.pc);
                let addr = base.wrapping_add(self.reg_y as u16);
                addr
            }
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.pc);

                let ptr: u8 = (base as u8).wrapping_add(self.reg_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);

                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.pc);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.reg_y as u16);

                deref
            }
            AddressingMode::Indirect => {
                panic!("mode: {:?} is only used for JMP instruction and should not be used with this function", mode);
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    /// This function runs the CPU and provides a callback function that is called every step of
    /// the way.
    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);
            if self.run_step() {
                return;
            }
        }
    }

    pub fn run_step(&mut self) -> bool {
        let op_code = self.mem_read(self.pc);

        let op = op_codes::OP_CODES[op_code as usize].unwrap_or_else(|| {
            panic!("Unimplemented opscode: {:02X}", op_code);
        });

        self.pc += 1;
        let initial_pc = self.pc;

        match op.code {
            op_codes::ADC_IMMEDIATE
            | op_codes::ADC_ZERO_PAGE
            | op_codes::ADC_ZERO_PAGE_X
            | op_codes::ADC_ABSOLUTE
            | op_codes::ADC_ABSOLUTE_X
            | op_codes::ADC_ABSOLUTE_Y
            | op_codes::ADC_INDIRECT_X
            | op_codes::ADC_INDIRECT_Y => {
                self.adc(op.addr_mode);
            }
            op_codes::AND_IMMEDIATE
            | op_codes::AND_ZERO_PAGE
            | op_codes::AND_ZERO_PAGE_X
            | op_codes::AND_ABSOLUTE
            | op_codes::AND_ABSOLUTE_X
            | op_codes::AND_ABSOLUTE_Y
            | op_codes::AND_INDIRECT_X
            | op_codes::AND_INDIRECT_Y => {
                self.and(op.addr_mode);
            }
            op_codes::ASL_ACCUMULATOR => {
                self.asl_acc();
            }
            op_codes::ASL_ZERO_PAGE
            | op_codes::ASL_ZERO_PAGE_X
            | op_codes::ASL_ABSOLUTE
            | op_codes::ASL_ABSOLUTE_X => {
                self.asl(op.addr_mode);
            }
            op_codes::BCC => {
                self.branch(!self.status.contains(StatusFlags::CARRY));
            }
            op_codes::BCS => {
                self.branch(self.status.contains(StatusFlags::CARRY));
            }
            op_codes::BEQ => {
                self.branch(self.status.contains(StatusFlags::ZERO));
            }
            op_codes::BIT_ZERO_PAGE | op_codes::BIT_ABSOLUTE => self.bit(op.addr_mode),
            op_codes::BMI => {
                self.branch(self.status.contains(StatusFlags::NEGATIVE));
            }
            op_codes::BNE => {
                self.branch(!self.status.contains(StatusFlags::ZERO));
            }
            op_codes::BPL => {
                self.branch(!self.status.contains(StatusFlags::NEGATIVE));
            }
            op_codes::BRK => return true,
            op_codes::BVC => {
                self.branch(!self.status.contains(StatusFlags::OVERFLOW));
            }
            op_codes::BVS => {
                self.branch(self.status.contains(StatusFlags::OVERFLOW));
            }
            op_codes::CLC => {
                self.status.remove(StatusFlags::CARRY);
            }
            op_codes::CLD => {
                self.status.remove(StatusFlags::DECIMAL_MODE);
            }
            op_codes::CLI => {
                self.status.remove(StatusFlags::INTERRUPT_DISABLE);
            }
            op_codes::CLV => {
                self.status.remove(StatusFlags::OVERFLOW);
            }
            op_codes::CMP_IMMEDIATE
            | op_codes::CMP_ZERO_PAGE
            | op_codes::CMP_ZERO_PAGE_X
            | op_codes::CMP_ABSOLUTE
            | op_codes::CMP_ABSOLUTE_X
            | op_codes::CMP_ABSOLUTE_Y
            | op_codes::CMP_INDIRECT_X
            | op_codes::CMP_INDIRECT_Y => {
                self.compare(&op.addr_mode, self.reg_a);
            }
            op_codes::CPX_IMMEDIATE | op_codes::CPX_ZERO_PAGE | op_codes::CPX_ABSOLUTE => {
                self.compare(&op.addr_mode, self.reg_x);
            }
            op_codes::CPY_IMMEDIATE | op_codes::CPY_ZERO_PAGE | op_codes::CPY_ABSOLUTE => {
                self.compare(&op.addr_mode, self.reg_y);
            }
            op_codes::DEC_ZERO_PAGE
            | op_codes::DEC_ZERO_PAGE_X
            | op_codes::DEC_ABSOLUTE
            | op_codes::DEC_ABSOLUTE_X => {
                self.dec(&op.addr_mode);
            }
            op_codes::DEX => {
                self.dex();
            }
            op_codes::DEY => {
                self.dey();
            }
            op_codes::EOR_IMMEDIATE
            | op_codes::EOR_ZERO_PAGE
            | op_codes::EOR_ZERO_PAGE_X
            | op_codes::EOR_ABSOLUTE
            | op_codes::EOR_ABSOLUTE_X
            | op_codes::EOR_ABSOLUTE_Y
            | op_codes::EOR_INDIRECT_X
            | op_codes::EOR_INDIRECT_Y => {
                self.eor(&op.addr_mode);
            }
            op_codes::INC_ZERO_PAGE
            | op_codes::INC_ZERO_PAGE_X
            | op_codes::INC_ABSOLUTE
            | op_codes::INC_ABSOLUTE_X => {
                self.inc(&op.addr_mode);
            }
            op_codes::INX => {
                self.inx();
            }
            op_codes::INY => {
                self.iny();
            }
            op_codes::JMP_ABSOLUTE | op_codes::JMP_INDIRECT => {
                self.jmp(op.addr_mode);
            }
            op_codes::JSR => {
                self.jsr();
            }
            op_codes::LDA_IMMEDIATE
            | op_codes::LDA_ZERO_PAGE
            | op_codes::LDA_ZERO_PAGE_X
            | op_codes::LDA_ABSOLUTE
            | op_codes::LDA_ABSOLUTE_X
            | op_codes::LDA_ABSOLUTE_Y
            | op_codes::LDA_INDIRECT_X
            | op_codes::LDA_INDIRECT_Y => {
                self.lda(op.addr_mode);
            }
            op_codes::LDX_IMMEDIATE
            | op_codes::LDX_ZERO_PAGE
            | op_codes::LDX_ZERO_PAGE_Y
            | op_codes::LDX_ABSOLUTE
            | op_codes::LDX_ABSOLUTE_Y => {
                self.ldx(op.addr_mode);
            }
            op_codes::LDY_IMMEDIATE
            | op_codes::LDY_ZERO_PAGE
            | op_codes::LDY_ZERO_PAGE_X
            | op_codes::LDY_ABSOLUTE
            | op_codes::LDY_ABSOLUTE_X => {
                self.ldy(op.addr_mode);
            }
            op_codes::LSR_ACCUMULATOR => {
                self.lsr_acc();
            }
            op_codes::LSR_ZERO_PAGE
            | op_codes::LSR_ZERO_PAGE_X
            | op_codes::LSR_ABSOLUTE
            | op_codes::LSR_ABSOLUTE_X => {
                self.lsr(&op.addr_mode);
            }
            op_codes::NOP => {}
            op_codes::ORA_IMMEDIATE
            | op_codes::ORA_ZERO_PAGE
            | op_codes::ORA_ZERO_PAGE_X
            | op_codes::ORA_ABSOLUTE
            | op_codes::ORA_ABSOLUTE_X
            | op_codes::ORA_ABSOLUTE_Y
            | op_codes::ORA_INDIRECT_X
            | op_codes::ORA_INDIRECT_Y => {
                self.ora(&op.addr_mode);
            }
            op_codes::PHA => {
                self.pha();
            }
            op_codes::PHP => {
                self.php();
            }
            op_codes::PLA => {
                self.pla();
            }
            op_codes::PLP => {
                self.plp();
            }
            op_codes::ROL_ACCUMULATOR => {
                self.rol_acc();
            }
            op_codes::ROL_ZERO_PAGE
            | op_codes::ROL_ZERO_PAGE_X
            | op_codes::ROL_ABSOLUTE
            | op_codes::ROL_ABSOLUTE_X => {
                self.rol(&op.addr_mode);
            }
            op_codes::ROR_ACCUMULATOR => {
                self.ror_acc();
            }
            op_codes::ROR_ZERO_PAGE
            | op_codes::ROR_ZERO_PAGE_X
            | op_codes::ROR_ABSOLUTE
            | op_codes::ROR_ABSOLUTE_X => {
                self.ror(&op.addr_mode);
            }
            op_codes::RTI => {
                self.rti();
            }
            op_codes::RTS => {
                self.rts();
            }
            op_codes::SBC_IMMEDIATE
            | op_codes::SBC_ZERO_PAGE
            | op_codes::SBC_ZERO_PAGE_X
            | op_codes::SBC_ABSOLUTE
            | op_codes::SBC_ABSOLUTE_X
            | op_codes::SBC_ABSOLUTE_Y
            | op_codes::SBC_INDIRECT_X
            | op_codes::SBC_INDIRECT_Y => {
                self.sbc(&op.addr_mode);
            }
            op_codes::SEC => {
                self.status.insert(StatusFlags::CARRY);
            }
            op_codes::SED => {
                self.status.insert(StatusFlags::DECIMAL_MODE);
            }
            op_codes::SEI => {
                self.status.insert(StatusFlags::INTERRUPT_DISABLE);
            }
            op_codes::STA_ZERO_PAGE
            | op_codes::STA_ZERO_PAGE_X
            | op_codes::STA_ABSOLUTE
            | op_codes::STA_ABSOLUTE_X
            | op_codes::STA_ABSOLUTE_Y
            | op_codes::STA_INDIRECT_X
            | op_codes::STA_INDIRECT_Y => {
                self.sta(op.addr_mode);
            }
            op_codes::STX_ZERO_PAGE | op_codes::STX_ZERO_PAGE_Y | op_codes::STX_ABSOLUTE => {
                self.stx(op.addr_mode);
            }
            op_codes::STY_ZERO_PAGE | op_codes::STY_ZERO_PAGE_X | op_codes::STY_ABSOLUTE => {
                self.sty(op.addr_mode);
            }
            op_codes::TAX => self.tax(),
            op_codes::TAY => self.tay(),
            op_codes::TSX => self.tsx(),
            op_codes::TXA => self.txa(),
            op_codes::TXS => self.txs(),
            op_codes::TYA => self.tya(),
            _ => panic!("Unimplemented opscode: {:02X}", op_code),
        }

        if initial_pc == self.pc {
            self.pc += op.size - 1;
        }

        return false;
    }

    fn plp(&mut self) {
        let v = self.pop_stack();
        self.status = StatusFlags::from_bits_truncate(v);
    }

    fn pla(&mut self) {
        let v = self.pop_stack();
        self.reg_a = v;
        self.update_zero_and_negative_flags(v);
    }

    fn php(&mut self) {
        self.push_stack(self.status.bits());
    }

    fn pha(&mut self) {
        self.push_stack(self.reg_a);
    }

    fn ora(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);
        self.reg_a = value | self.reg_a;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn jsr(&mut self) {
        let return_address = self.pc + 2 - 1;
        self.push_stack_u16(return_address);
        self.pc = self.mem_read_u16(self.pc);
    }

    fn jmp(&mut self, addr_mode: AddressingMode) {
        match addr_mode {
            AddressingMode::Absolute => {
                let mem_address = self.mem_read_u16(self.pc);
                self.pc = mem_address;
            }
            AddressingMode::Indirect => {
                let mem_address = self.mem_read_u16(self.pc);

                // Do this due to a bug in the 6502
                let indirect_ref = if mem_address & 0x00FF == 0x00FF {
                    let lo = self.mem_read(mem_address);
                    let hi = self.mem_read(mem_address & 0xFF00);
                    (hi as u16) << 8 | (lo as u16)
                } else {
                    self.mem_read_u16(mem_address)
                };

                self.pc = indirect_ref;
            }
            _ => panic!("Unimplemented addressing mode: {:?}", addr_mode),
        }
    }

    fn inx(&mut self) {
        self.reg_x = self.reg_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.reg_x);
    }

    fn iny(&mut self) {
        self.reg_y = self.reg_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.reg_y);
    }

    fn inc(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_op_addr(addr_mode);
        let data = self.mem_read(addr);
        let new_value = data.wrapping_add(1);
        self.mem_write(addr, new_value);
    }

    fn eor(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);
        self.reg_a = value ^ self.reg_a;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn compare(&mut self, mode: &AddressingMode, cmp_v: u8) {
        let addr = self.get_op_addr(mode);
        let data = self.mem_read(addr);
        if data <= cmp_v {
            self.status.insert(StatusFlags::CARRY);
        } else {
            self.status.remove(StatusFlags::CARRY);
        }

        self.update_zero_and_negative_flags(cmp_v.wrapping_sub(data));
    }

    fn dey(&mut self) {
        let new_value = self.reg_y.wrapping_sub(1);
        self.reg_y = new_value;
        self.update_zero_and_negative_flags(new_value);
    }

    fn dex(&mut self) {
        let new_value = self.reg_x.wrapping_sub(1);
        self.reg_x = new_value;
        self.update_zero_and_negative_flags(new_value);
    }

    fn dec(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);
        let new_value = value.wrapping_sub(1);
        self.mem_write(addr, new_value);
        self.update_zero_and_negative_flags(new_value);
    }

    fn add_to_register_a(&mut self, data: u8) {
        let sum = self.reg_a as u16
            + data as u16
            + (if self.status.contains(StatusFlags::CARRY) {
                1
            } else {
                0
            }) as u16;

        let carry = sum > 0xff;

        if carry {
            self.status.insert(StatusFlags::CARRY);
        } else {
            self.status.remove(StatusFlags::CARRY);
        }

        let result = sum as u8;

        if (data ^ result) & (result ^ self.reg_a) & 0x80 != 0 {
            self.status.insert(StatusFlags::OVERFLOW);
        } else {
            self.status.remove(StatusFlags::OVERFLOW)
        }

        self.reg_a = result;
    }

    fn bit(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);

        let result = self.reg_a & value == 0;
        if result {
            self.status.insert(StatusFlags::ZERO);
        } else {
            self.status.remove(StatusFlags::ZERO);
        }

        if value & 0b0100_0000 != 0 {
            self.status.insert(StatusFlags::OVERFLOW);
        } else {
            self.status.remove(StatusFlags::OVERFLOW);
        }

        if value & 0b1000_0000 != 0 {
            self.status.insert(StatusFlags::NEGATIVE);
        } else {
            self.status.remove(StatusFlags::NEGATIVE);
        }
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let offset = self.mem_read(self.pc) as i8;
            self.pc = self.pc.wrapping_add(offset as u16).wrapping_add(1);
        }
    }

    fn and(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);
        self.reg_a = value & self.reg_a;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn adc(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);
        self.add_to_register_a(value);
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn asl_acc(&mut self) {
        self.update_carry(self.reg_a);
        self.reg_a = self.reg_a << 1;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn asl(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);
        let new_value = value << 1;

        self.update_carry(value);
        self.mem_write(addr, new_value);
        self.update_zero_and_negative_flags(new_value);
    }

    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_zero_and_negative_flags(self.reg_x);
    }

    fn tay(&mut self) {
        self.reg_y = self.reg_a;
        self.update_zero_and_negative_flags(self.reg_y);
    }

    fn tsx(&mut self) {
        self.reg_x = self.sp;
        self.update_zero_and_negative_flags(self.reg_x);
    }

    fn txa(&mut self) {
        self.reg_a = self.reg_x;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn txs(&mut self) {
        self.sp = self.reg_x;
    }

    fn tya(&mut self) {
        self.reg_a = self.reg_y;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn lda(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);

        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn ldx(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);

        self.reg_x = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn ldy(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let value = self.mem_read(addr);

        self.reg_y = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn sta(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        self.mem_write(addr, self.reg_a);
    }

    fn stx(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        self.mem_write(addr, self.reg_x);
    }

    fn sty(&mut self, addr_mode: AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        self.mem_write(addr, self.reg_y);
    }

    fn rol_acc(&mut self) {
        let v = self.reg_a;
        let mut new_value = v << 1;
        if self.status.contains(StatusFlags::CARRY) {
            new_value |= 0b0000_0001;
        }
        self.update_carry(v);
        self.reg_a = new_value;
        self.update_zero_and_negative_flags(new_value);
    }

    fn rol(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let v = self.mem_read(addr);

        let mut new_value = v << 1;
        if self.status.contains(StatusFlags::CARRY) {
            new_value |= 0b0000_0001;
        }
        self.update_carry(v);
        self.mem_write(addr, new_value);
        self.update_zero_and_negative_flags(new_value);
    }

    fn lsr_acc(&mut self) {
        let v = self.reg_a;
        let new_value = v >> 1;
        self.update_carry(v.reverse_bits());
        self.reg_a = new_value;
        self.update_zero_and_negative_flags(new_value);
    }

    fn lsr(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let v = self.mem_read(addr);
        let new_value = v >> 1;
        self.update_carry(v.reverse_bits());
        self.mem_write(addr, new_value);
        self.update_zero_and_negative_flags(new_value);
    }

    fn ror_acc(&mut self) {
        let v = self.reg_a;
        let mut new_value = v >> 1;
        if self.status.contains(StatusFlags::CARRY) {
            new_value |= 0b1000_0000;
        }
        self.update_carry(v.reverse_bits());
        self.reg_a = new_value;
        self.update_zero_and_negative_flags(new_value);
    }

    fn ror(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let v = self.mem_read(addr);

        let mut new_value = v >> 1;
        if self.status.contains(StatusFlags::CARRY) {
            new_value |= 0b1000_0000;
        }
        self.update_carry(v.reverse_bits());
        self.mem_write(addr, new_value);
        self.update_zero_and_negative_flags(new_value);
    }

    fn rti(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.pop_stack());
        self.pc = self.pop_stack_u16();
    }

    fn rts(&mut self) {
        self.pc = self.pop_stack_u16() + 1;
    }

    fn sbc(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_op_addr(&addr_mode);
        let data = self.mem_read(addr);
        self.add_to_register_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::ADC_IMMEDIATE, 0x42, op_codes::BRK]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.reg_a, 0x42);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(!cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::OVERFLOW));
    }

    #[test]
    fn test_adc_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::ADC_IMMEDIATE, 0x40, op_codes::BRK]);
        cpu.reset();
        cpu.reg_a = 0x40;
        cpu.run();

        assert_eq!(cpu.reg_a, 0x80);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(!cpu.status.contains(StatusFlags::CARRY));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(cpu.status.contains(StatusFlags::OVERFLOW));
    }

    #[test]
    fn test_adc_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::ADC_IMMEDIATE, 0x80, op_codes::BRK]);
        cpu.reset();
        cpu.reg_a = 0x80;
        cpu.run();

        assert_eq!(cpu.reg_a, 0x00);
        assert!(cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(cpu.status.contains(StatusFlags::OVERFLOW));
    }

    #[test]
    fn test_and_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::AND_IMMEDIATE, 0b10101110, op_codes::BRK]);
        cpu.reset();
        cpu.reg_a = 0b11111111;
        cpu.run();

        assert_eq!(cpu.reg_a, 0b10101110);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::ASL_ACCUMULATOR, op_codes::BRK]);
        cpu.reset();
        cpu.reg_a = 0b11101101;
        cpu.run();

        assert_eq!(cpu.reg_a, 0b11011010);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_asl_absolute() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::ASL_ABSOLUTE, 0x10, op_codes::BRK]);
        cpu.reset();
        cpu.mem_write(0x10, 0b11101101);
        cpu.run();

        assert_eq!(cpu.mem_read(0x10), 0b11011010);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_bcc_carry_set() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::BCC, 0x01, op_codes::BRK, op_codes::BRK]);
        cpu.reset();
        cpu.status.insert(StatusFlags::CARRY);
        cpu.run();

        assert_eq!(cpu.pc, 0x3 + PC_OFFSET);
    }

    #[test]
    fn test_bcc_carry_not_set() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::BCC, 0x01, op_codes::BRK, op_codes::BRK]);
        cpu.reset();
        cpu.status.remove(StatusFlags::CARRY);
        cpu.run();

        assert_eq!(cpu.pc, 0x4 + PC_OFFSET);
    }

    #[test]
    fn test_bcs_carry_set() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::BCS, 0x01, op_codes::BRK, op_codes::BRK]);
        cpu.reset();
        cpu.status.insert(StatusFlags::CARRY);
        cpu.run();

        assert_eq!(cpu.pc, 0x4 + PC_OFFSET);
    }

    #[test]
    fn test_bcs_carry_not_set() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::BCS, 0x01, op_codes::BRK, op_codes::BRK]);
        cpu.reset();
        cpu.status.remove(StatusFlags::CARRY);
        cpu.run();

        assert_eq!(cpu.pc, 0x3 + PC_OFFSET);
    }

    #[test]
    fn test_beq_zero_set() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::BEQ, 0x01, op_codes::BRK, op_codes::BRK]);
        cpu.reset();
        cpu.status.insert(StatusFlags::ZERO);
        cpu.run();

        assert_eq!(cpu.pc, 0x04 + PC_OFFSET);
    }

    #[test]
    fn test_beq_zero_not_set() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::BEQ, 0x01, op_codes::BRK, op_codes::BRK]);
        cpu.reset();
        cpu.status.remove(StatusFlags::ZERO);
        cpu.run();

        assert_eq!(cpu.pc, 0x03 + PC_OFFSET);
    }

    #[test]
    #[ignore]
    fn test_bit_absolute() {
        todo!();
    }

    #[test]
    #[ignore]
    fn test_bmi_negative_set() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_bmi_negative_not_set() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_bne_zero_set() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_bne_zero_not_set() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_bpl_negative_set() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_bpl_negative_not_set() {
        todo!()
    }

    #[test]
    fn test_brk() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::BRK]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.pc, 0x1 + PC_OFFSET);
    }

    #[test]
    #[ignore]
    fn test_bvc_overflow_set() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_bvc_overflow_not_set() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_bvs_overflow_set() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_bvs_overflow_not_set() {
        todo!()
    }

    #[test]
    fn test_clc() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::CLC, op_codes::BRK]);
        cpu.reset();
        cpu.status.insert(StatusFlags::CARRY);
        cpu.run();

        assert!(!cpu.status.contains(StatusFlags::CARRY));
    }

    #[test]
    fn test_cld() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::CLD, op_codes::BRK]);
        cpu.reset();
        cpu.status.insert(StatusFlags::DECIMAL_MODE);
        cpu.run();

        assert!(!cpu.status.contains(StatusFlags::DECIMAL_MODE));
    }

    #[test]
    fn test_cli() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::CLI, op_codes::BRK]);
        cpu.reset();
        cpu.status.insert(StatusFlags::INTERRUPT_DISABLE);
        cpu.run();

        assert!(!cpu.status.contains(StatusFlags::INTERRUPT_DISABLE));
    }

    #[test]
    fn test_clv() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::CLV, op_codes::BRK]);
        cpu.reset();
        cpu.status.insert(StatusFlags::OVERFLOW);
        cpu.run();

        assert!(!cpu.status.contains(StatusFlags::OVERFLOW));
    }

    #[test]
    #[ignore]
    fn test_cmp_immediate() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_cpx_immediate() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_cpy_immediate() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_dec_absolute() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_dex() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_dey() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_eor_immediate() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_inc_absolute() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_inx() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_iny() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_jmp_absolute() {
        todo!();
    }

    #[test]
    #[ignore]
    fn test_jsr() {
        todo!();
    }

    #[test]
    fn test_ror_accumulator_carry_set() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::ROR_ACCUMULATOR, op_codes::BRK]);
        cpu.reset();
        cpu.reg_a = 0b11101100;
        cpu.status.insert(StatusFlags::CARRY);
        cpu.run();

        assert_eq!(cpu.reg_a, 0b11110110);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::CARRY));
    }

    #[test]
    fn test_ror_accumulator_carry_not_set() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::ROR_ACCUMULATOR, op_codes::BRK]);
        cpu.reset();
        cpu.reg_a = 0b11101101;
        cpu.status.remove(StatusFlags::CARRY);
        cpu.run();

        assert_eq!(cpu.reg_a, 0b01110110);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::LDA_IMMEDIATE, 0x42, op_codes::BRK]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.reg_a, 0x42);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![op_codes::LDA_IMMEDIATE, 0x00, op_codes::BRK]);
        cpu.reset();
        cpu.run();
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![
            op_codes::LDA_IMMEDIATE,
            0x55,
            op_codes::TAX,
            op_codes::BRK,
        ]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.reg_x, 0x55);
    }

    #[test]
    fn test_0xe8_inx_increment_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![
            op_codes::LDA_IMMEDIATE,
            0x00,
            op_codes::INX,
            op_codes::BRK,
        ]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.reg_x, 0x01);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.reg_x = 0xff;
        cpu.run();

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load(vec![0xa5, 0x10, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.reg_a, 0x55);
    }
}
