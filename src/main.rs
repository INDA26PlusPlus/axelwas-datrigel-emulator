fn get_funct3(instruction: u32) -> u32 {
    (instruction & 0b11100000_00000000) >> 11
}

fn get_funct12(instruction: u32) -> u32 {
    (instruction & (0b1111_1111_1111_0000_0000_0000_0000_0000)) >> 20
}

fn get_rs1(instruction: u32) -> u32 {
    (instruction & (0b1111_1000_0000_0000_0000)) >> 15
}

fn get_rd(instruction: u32) -> u32 {
    (instruction & (0b1111_1000_0000)) >> 7
}

fn get_imm20(instruction: u32) -> u32 {
    (instruction & 0b1111_1111_1111_1111_1111_0000_0000_0000) >> 12
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
    
    fn run_instruction(&mut self) {
        if (self.registers.pc % 4) != 0 {
            panic!("TODO: implement exception");
        }
        let instruction = self.read_word(self.registers.pc);
        let opcode = instruction & 0x0000_007F;
        

        match opcode {
            0b0110111 => return self.lui(instruction),
            0b0010111 => todo!("AUIPC"),
            0b1101111 => todo!("JAL"),
            0b1100111 => todo!("JALR"),
            _ => ()
            
            
        }

        match opcode {
            0b1100011 => todo!("conditional branches"),
            0b0000011 => todo!("load instructions"),
            0b0100011 => todo!("store instructions"),
            0b0010011 => todo!("integer register immediate instructions"),
            0b0110011 => todo!("integer register register instructions"),
            0b0001111 => return self.memory_ordering(instruction),
            0b1110011 => return self.system_instructions(instruction),
            _ => panic!("NOT IMPLMENETED OPCODE"),
        }
    }
}

fn main() {
    let rom = [];
    let mut computer = State::new(&rom);
    loop {
        computer.run_instruction();
    }
}

#[cfg(test)]
mod tests;