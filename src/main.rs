//////////////////////////////
////// driver code
//////////////////////////////

mod vm;
use crate::vm::{terminal_io, VM};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    terminal_io::setup_terminal();

    let mut vm = VM::new();
    vm.read_program(args.get(1).expect("No program file given"));
    vm.execute();
}
