/*

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation; either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see https://www.gnu.org/licenses/.

© 2024 dogeystamp <dogeystamp@disroot.org>
*/

//////////////////////////////
////// memory and memory-mapped registers
//////////////////////////////

////////////////
// memory interface
////////////////

use super::terminal_io;

pub const MEM_SIZE: usize = 1 << 16;

pub struct Memory<'a> {
    data: [u16; MEM_SIZE],
    io: &'a mut dyn terminal_io::KeyboardIO,
}

impl Memory<'_> {
    pub fn new(keyboard_io: &mut dyn terminal_io::KeyboardIO) -> Memory {
        Memory {
            data: [0; MEM_SIZE],
            io: keyboard_io,
        }
    }

    pub fn set_mem(&mut self, addr: u16, val: u16) {
        self.data[addr as usize] = val;
    }

    pub fn get_mem(&mut self, addr: u16) -> u16 {
        if addr >= 0xFE00 {
            return match addr {
                0xFE00 => {
                    if self.io.check_key() {
                        1 << 15
                    } else {
                        0
                    }
                }
                0xFE02 => {
                    let key = match self.io.get_key() {
                        Some(key) => key as u16,
                        None => self.data[0xFE02],
                    };
                    return key;
                }
                _ => unimplemented!("mem-map: {:#X}", addr),
            };
        }
        return self.data[addr as usize];
    }
}
