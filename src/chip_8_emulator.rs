pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 12],
    stack_pointer: u8,
    pc: u16,
}

impl Chip8 {
    pub fn initialize() -> Self {
        Self {
            registers: [0; 16],
            memory: [0x00; 4096],
            stack: [0x0000; 12],
            stack_pointer: 0,
            pc: 0,
        }
    }

    fn read_op(&mut self) -> u16 {
        let op_code =
            ((self.memory[self.pc as usize] as u16) << 8) | (self.memory[self.pc as usize] as u16);

        self.pc += 2;

        op_code
    }

    pub fn run(&mut self) {
        loop {
            let op_code = self.read_op();

            let x = ((op_code & 0x0F00) >> 8) as u8;
            let y = ((op_code & 0x00F0) >> 4) as u8;
            let nn = (op_code & 0x00FF) as u8;
            let nnn = (op_code & 0x0FFF) as u8;

            match op_code {
                0x0000 => return,
                0x00EE => self.func_ret(),
                0x1000..=0x1FFF => self.jmp(nnn),
                0x2000..=0x2FFF => self.call(nnn),
                0x3000..=0x3FFF => self.je(x, nn),

                _ => println!("Invalid or Unimplemented opcode: {:04x}", op_code),
            }
        }
    }

    fn func_ret(&self) {}

    fn jmp(&mut self, addr: u8) {
        self.pc = addr as u16;
    }

    fn call(&mut self, addr: u8) {}

    fn je(&mut self, reg: u8, cmp: u8) {}
}
