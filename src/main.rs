//////////////////////////////
////// driver code
//////////////////////////////

mod vm;
use crate::vm::VM;
use std::env;

fn main() {
    let args : Vec<_> = env::args().collect();

    let mut vm = VM::new();
    vm.read_program(args.get(1).expect("No program file given"));
    vm.execute();
}
