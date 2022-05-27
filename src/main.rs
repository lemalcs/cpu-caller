struct CPU {
    registers: [u8; 16], // (container of data that the CPU accesses directly
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16], // specialized memory for storing addresses
    stack_pointer: usize,
}

impl CPU {
    /// Reads an opcode from memory by combining two values into a single u16 value
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // Move the value of ´óp_byte1´ 8 places to the left 
        // and allocate the value of ´op_byte2´ to the right
        // in order to create and u16 value (16 bits)
        op_byte1 << 8 | op_byte2 // same as (op_byte1 << 8) | op_byte2
    }

    /// Calls a function
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        // ´position_in_memory´ is two bytes higher than the calling location
        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1; // prevent memory to be overwritten
        self.position_in_memory = addr as usize;
    }

    /// Returns from a function
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize; // set memory asdress to the previous CALL opcode
    }

    /// Adds two numbers located in registers of CPU
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        // the last register is termed *carry flag* that indicates if an operation
        // has overflowed
        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    // Call functions exeuting them in the CPU emulator
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            // get memory address from opcode
            let nnn = opcode & 0xFFF;

            match(c, x, y, d) {
                ( 0, 0, 0, 0) => { return; },
                ( 0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }

        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    
    // Load a a function into memory
    // this usually is done with a programming language
    // but here it is done with hard-coded operation codes
    let mem = &mut cpu.memory;
    mem[0x000] = 0x21;  mem[0x001] = 0x00;
    mem[0x002] = 0x21;  mem[0x003] = 0x00;
    mem[0x004] = 0x00;  mem[0x005] = 0x00;
    
    mem[0x100] = 0x80;  mem[0x101] = 0x14;
    mem[0x102] = 0x80;  mem[0x103] = 0x14;
    mem[0x104] = 0x00;  mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);

}
