#![allow(dead_code)]
#![allow(unused_variables)]

const MEM_SIZE: usize = 1 << 16;
const PC_START: usize = 0x3000;

enum OpCodes {
    // branch
    BR = 0,
    // add
    ADD,
    // load
    LD,
    // store
    ST,
    // jump register
    JSR,
    // bitwise and
    AND,
    // load register
    LDR,
    // store registerj
    STR,
    // return from interrupt (unused)
    RTI,
    // bitwise not
    NOT,
    // load indirect
    LDI,
    // store indirect
    STI,
    // jump
    JMP,
    // reserved (unused)
    RES,
    // load effective address
    LEA,
    // trap
    TRAP,
}

struct Registers {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    pc: u16,
    cond: u16,
    count: u16,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: PC_START as u16,
            cond: 0,
            count: 0,
        }
    }

    fn register_reference(&mut self, idx: u16) -> &mut u16 {
        match idx {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            4 => &mut self.r4,
            5 => &mut self.r5,
            6 => &mut self.r6,
            7 => &mut self.r7,
            8 => &mut self.pc,
            9 => &mut self.cond,
            _ => panic!("Invalid register {}", idx),
        }
    }

    fn set_reg(&mut self, idx: u16, val: u16) {
        *self.register_reference(idx) = val;
    }

    fn get_reg(&mut self, idx: u16) -> u16 {
        let reg = &*self.register_reference(idx);
        *reg
    }
}

struct VM {
    mem: [u16; MEM_SIZE]
}

fn main() {
    println!("Hello, world!");

    let mut regs = Registers::new();
    println!("was {}", regs.get_reg(0));
    regs.set_reg(0, 3);
    println!("set to {}", regs.get_reg(0));
}
