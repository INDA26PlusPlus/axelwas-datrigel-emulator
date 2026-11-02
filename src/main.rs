struct Registers {
    x_registers: [u32; 31],
    pc: u32
}

impl Registers {
    fn get(&self, register: u32) -> u32 {
        match register {
            0 => 0,
            x if x < 32 => self.x_registers[x as usize],
            _ => panic!("Illegal register")
        }
    }
    fn write(&mut self, register: u32, value: u32) {
        match register {
            0 => (),
            x if x < 32 => self.x_registers[x as usize] = value,
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

struct State {
    registers: Registers,
    ram: [u8; 0xFFFFFF], // 2 ^ 24 = 16.77 MB
    rom: [u8; 0xFFFFFF]
}

impl State {
    fn new(rom: [u8; 0xFFFFFF]) -> Self {
        Self {
            registers: Registers::new(),
            ram: [0; 0xFFFFFF],
            rom
        }
    }
    fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            a if a < 0xFFFFFF => self.ram[a as usize],
            a if a > 0x0800_0000 && a < 0x08FFFFFF => self.rom[a as usize - 0x0800_0000],
            _ => 0
        }
    }

    fn read_word(&self, addr: u32) -> u32 {
        u32::from_le_bytes([self.read_byte(addr), 
            self.read_byte(addr + 1), 
            self.read_byte(addr + 2), 
            self.read_byte(addr + 3)])
    }

    fn write_byte(&mut self, addr: u32, value: u8) {
        match addr {
            a if a < 0xFFFFFF => self.ram[a as usize] = value,
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
}

fn main() {
    let computer = State::new([0; 0xFFFFFF]);
    println!("Hello, world!");
}
