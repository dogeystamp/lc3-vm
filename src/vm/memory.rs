//////////////////////////////
////// memory and memory-mapped registers
//////////////////////////////

////////////////
// memory interface
////////////////

use super::terminal_io;

pub const MEM_SIZE: usize = 1 << 16;

pub struct Memory {
    data: [u16; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; MEM_SIZE],
        }
    }

    pub fn set_mem(&mut self, addr: u16, val: u16) {
        self.data[addr as usize] = val;
    }

    pub fn get_mem(&self, addr: u16) -> u16 {
        if addr >= 0xFE00 {
            match addr {
                _ => unimplemented!("mem-map: {:#X}", addr),
            }
        }
        return self.data[addr as usize];
    }
}
