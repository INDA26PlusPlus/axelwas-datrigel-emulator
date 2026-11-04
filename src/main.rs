fn get_funct3(instruction: u32) -> u32 {
    (instruction & 0b11100000_00000000) >> 11
}

fn get_funct12(instruction: u32) -> u32 {
    (instruction & (0b1111_1111_1111_0000_0000_0000_0000_0000)) >> 20
}

fn get_rs1(instruction: u32) -> u32 {
    (instruction & (0b1111_1000_0000_0000_0000)) >> 15
}

fn get_rs2(instruction: u32) -> u32 {
    (instruction & (0b1_1111_0000_0000_0000_0000_0000)) >> 15
}

fn get_rd(instruction: u32) -> u32 {
    (instruction & (0b1111_1000_0000)) >> 7
}

fn get_imm20(instruction: u32) -> u32 {
    (instruction & 0b1111_1111_1111_1111_1111_0000_0000_0000) >> 12
}

fn get_imm12(instruction: u32) -> u32 {
    (instruction & 0b1111_1111_1111_0000_0000_0000_0000_0000) >> 20
}

fn add_12i(to_add: u32, value: u32) -> u32 {
    if (to_add & 0b1000_0000_0000) != 0 {
        // value negative
        let actually_add = to_add | 0b1111_1111_1111_1111_1111_0000_0000_0000;
        value.overflowing_add(actually_add).0
    } else {
        value.overflowing_add(to_add).0
    }
}

#[derive(Debug)]
struct Registers {
    x_registers: [u32; 31],
    pc: u32
}

impl Registers {
    fn get(&self, register: u32) -> u32 {
        match register {
            0 => 0,
            x if x < 32 => self.x_registers[(x - 1) as usize],
            _ => panic!("Illegal register")
        }
    }
    fn write(&mut self, register: u32, value: u32) {
        match register {
            0 => (),
            x if x < 32 => self.x_registers[(x - 1) as usize] = value,
            _ => panic!("Illegal register")
        }
    }

    fn new() -> Self {
        Self {
            pc: 0x0800_0000, // default Linux start address
            x_registers: [0; 31]
        }
    }
}

#[derive(Debug)]
struct State<'a> {
    registers: Registers,
    ram: Vec<u8>, 
    rom: &'a [u8]
}

impl<'a> State<'a> {
    fn new(rom: &'a [u8]) -> State<'a> {
        let ram = vec![0u8; 0xFFFFFF]; // 16 MB bytes of ram
        State {
            registers: Registers::new(),
            ram,
            rom
        }
    }
    fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            a if a < 0xFFFFFF => self.ram.get(a as usize).map(|v| *v).unwrap_or(0),
            a if a >= 0x0800_0000 && a < 0x08FFFFFF => self.rom.get(a as usize - 0x0800_0000).map(|v| *v).unwrap_or(0),
            _ => 0
        }
    }

    fn read_word(&self, addr: u32) -> u32 {
        let bytes = [self.read_byte(addr), 
            self.read_byte(addr + 1), 
            self.read_byte(addr + 2), 
            self.read_byte(addr + 3)];
        u32::from_le_bytes(bytes)
    }

    fn write_byte(&mut self, addr: u32, value: u8) {
        match addr {
            a if (a as usize) < self.ram.len() => self.ram[a as usize] = value,
            _ => ()
        }
    }

    fn write_word(&mut self, addr: u32, value: u32) {
        let bytes = value.to_le_bytes();
        self.write_byte(addr, bytes[0]);
        self.write_byte(addr + 1, bytes[1]);
        self.write_byte(addr + 2, bytes[2]);
        self.write_byte(addr + 3, bytes[3]);
    }

    fn memory_ordering(&mut self, instruction: u32) {
        // Since this is single threaded all memory operations are in order. The instruction being valid however must be checked.

        let funct3 = get_funct3(instruction);

        if funct3 != 0 {
            panic!("Inavlid/not implemented memory ordering instruction");
        }
    }

    fn system_instructions(&mut self, instruction: u32) {
        let funct12 = get_funct12(instruction);
        let funct3 = get_funct3(instruction);
        let rs1 = get_rs1(instruction);
        let rd = get_rd(instruction);

        if funct3 != 0 || rs1 != 0 || rd != 0{
            panic!("Invalid system instruction");
        }

        match funct12 {
            1 => dbg!(&self.registers), // EBREAK
            0 => panic!("ECALL not valid, implemented non privellage instruction set"), // ECALL
            _ => panic!("Invalid system instruction")
        };
    }

    fn lui(&mut self, instruction: u32) {
        let imm = get_imm20(instruction);
        let rd = get_rd(instruction);

        dbg!(imm, rd);

        self.registers.write(rd, imm << 12);
    }
    
    fn store_instructions(&mut self, instruction: u32) {
        let imm = (get_imm12(instruction) & (0b1111_1110_0000)) | get_rd(instruction);
        let rs1 = get_rs1(instruction);
        let rs2 = get_rs2(instruction);
        let funct3 = get_funct3(instruction);
        
        let rs1_value = self.registers.get(rs1);
        let rs2_value = self.registers.get(rs2);

        let effective_address = add_12i(imm, rs1_value);


        match funct3 {
            0b000 => self.write_byte(effective_address, rs2_value.to_le_bytes()[0]), // SB
            0b001 => {
                self.write_byte(effective_address, rs2_value.to_le_bytes()[0]);
                self.write_byte(effective_address, rs2_value.to_le_bytes()[1]);
            }, // SH
            0b010 => self.write_word(effective_address, rs2_value), // SW
            _ => panic!("non implemented store instruction")
        }
    }

    fn run_instruction(&mut self) {
        if (self.registers.pc % 4) != 0 {
            panic!("TODO: implement exception");
        }
        let instruction = self.read_word(self.registers.pc);
        let opcode = instruction & 0x0000_007F;

        match opcode {
            0b0110111 => self.lui(instruction),
            0b0010111 => todo!("AUIPC"),
            0b1101111 => todo!("JAL"),
            0b1100111 => todo!("JALR"),
            0b1100011 => todo!("conditional branches"),
            0b0000011 => todo!("load instructions"),
            0b0100011 => self.store_instructions(instruction),
            0b0010011 => todo!("integer register immediate instructions"),
            0b0110011 => todo!("integer register register instructions"),
            0b0001111 => self.memory_ordering(instruction),
            0b1110011 => self.system_instructions(instruction),
            _ => panic!("NOT IMPLMENETED OPCODE"),
        };

        self.registers.pc = self.registers.pc + 4;
    }
}

fn main() {
    let prg = [0xB7, 0x10, 0x00, 0x00, 0x23, 0xA0, 0x10, 0x00, 0x23, 0xA2, 0x10, 
        0x00, 0x23, 0x94, 0x10, 0x00, 0x23, 0x85, 0x10, 0x00, 0xA3, 0x85, 0x10, 0x00];

    let mut computer = State::new(&prg);
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();
    computer.run_instruction();

    for b in (1<<12)..((1<<12) + 12) {
        let val = computer.read_byte(b);
        dbg!(b, val);
    }
}

#[cfg(test)]
mod tests;