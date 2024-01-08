//////////////////////////////
////// VM implemntation
//////////////////////////////

// NOTE
// trying this out https://www.pathsensitive.com/2023/12/should-you-split-that-file.html

#![allow(dead_code)]
#![allow(unused_variables)]

use byteorder::{BigEndian, ReadBytesExt};
use std::{fs::File, io::BufReader};

mod instruction;
mod memory;
pub mod terminal_io;

////////////////
// registers
////////////////

// condition flags (COND register)
enum CondFlags {
    // positive (P)
    POS = 1 << 0,
    // zero (Z)
    ZRO = 1 << 1,
    // negative (N)
    NEG = 1 << 2,
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

const PC_START: usize = 0x3000;

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

    /// Map an integer index to its register
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

    /// Sets COND register based on a specific register's value
    fn set_cond(&mut self, idx: u16) {
        let val = self.get_reg(idx);

        if (val as i16) > 0 {
            self.cond = CondFlags::POS as u16;
        } else if (val as i16) < 0 {
            self.cond = CondFlags::NEG as u16;
        } else if val == 0 {
            self.cond = CondFlags::ZRO as u16;
        }
    }

    fn set_reg_with_cond(&mut self, idx: u16, val: u16) {
        self.set_reg(idx, val);
        self.set_cond(idx);
    }
}

////////////////
// VM interface
////////////////

// NOTE
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// tl;dr the 'a is like a generic type name except it means that for some "lifetime" 'a, we will
// hold a reference to `io` and we promise not to not have the struct outlive it
// this helps prevent dangling references
pub struct VM<'a> {
    mem: memory::Memory<'a>,
    registers: Registers,
    running: bool,
    debug_state: DebugState,
}

impl VM<'_> {
    pub fn new(keyboard_io: &mut dyn terminal_io::KeyboardIO) -> VM {
        VM {
            mem: memory::Memory::new(keyboard_io),
            registers: Registers::new(),
            running: false,
            debug_state: DebugState::new(),
        }
    }

    pub fn read_program(&mut self, path: &String) {
        let f = File::open(path).expect("Could not open program file");
        let mut f = BufReader::new(f);

        // NOTE
        // LC-3 works with 16-bit words, in big endian.
        // Endianness is the order of bytes *within a word*.
        // Big endian is the "natural" order (left to right, start with most significant byte).
        // This means that 0x3000 is encoded as 30 00.
        // Little endian is commonly used on processors (see why at https://softwareengineering.stackexchange.com/questions/95556).
        // For example, 0x12345678 (if we assume 32-bit words) would be encoded as 78 56 34 12.
        // It turns out that the `hexdump` command uses 16-bit words and little-endian by default
        // (at least on my machine).
        // Therefore, it flips each pair of bytes.
        // Meanwhile, `hed` uses big-endian.
        // To make hexdump ignore words, pass the `-C` flag for a byte-by-byte output.
        let base_addr = f
            .read_u16::<BigEndian>()
            .expect("Program file could not be read");

        let mut addr = base_addr;
        loop {
            match f.read_u16::<BigEndian>() {
                Ok(word) => {
                    self.mem.set_mem(addr, word);
                    addr += 1;
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        panic!("Can not read instruction: {:?}", e);
                    }
                }
            }
        }
    }

    pub fn execute(&mut self) {
        self.running = true;

        while self.running {
            let instr = instruction::get_instruction(self);

            if self.debug_state.debugging {
                DebugState::print_state(self);
            }

            // NOTE
            // remember PC points to the *next* instruction at all times

            // disallow reading past memory bounds
            if self.registers.pc as usize == memory::MEM_SIZE - 1 {
                self.running = false
            } else {
                self.registers.pc += 1;
            }

            instruction::execute_instruction(self, instr);
        }
    }
}

////////////////
// debugging
////////////////

struct DebugState {
    debugging: bool,
}

impl DebugState {
    fn new() -> DebugState {
        DebugState {
            debugging: false,
        }
    }

    /// Print current VM state
    fn print_state(vm: &mut VM) {
        let instr = instruction::get_instruction(vm);
        let op_code = instruction::get_opcode(instr);

        println!("PC: {:#x}, op: {:?}, params: {:#x}", vm.registers.pc, op_code, instr & 0x7ff);
        for i in 0..=7 {
            println!("R{}: {:#x}", i, vm.registers.get_reg(i));
        }

        let mut condstr = String::new();
        let flags = ["P", "Z", "N"];
        for i in 0..=2 {
            if (1 << i) & vm.registers.cond == 1 {
                condstr.push_str(flags[i]);
            }
        }
        println!("COND: {:#x} ({})", vm.registers.cond, condstr);

        println!();
    }
}
