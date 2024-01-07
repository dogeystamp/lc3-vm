//////////////////////////////
////// driver code
//////////////////////////////

mod vm;
use crate::vm::{terminal_io, VM};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let term = terminal_io::TerminalIO::new();

    let mut vm = VM::new(&term);
    vm.read_program(args.get(1).expect("No program file given"));
    vm.execute();
}
