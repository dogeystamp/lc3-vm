//////////////////////////////
////// driver code
//////////////////////////////

mod vm;
use crate::vm::VM;

fn main() {
    let mut vm = VM::new();

    vm.read_program(&"programs/hello-world.obj".to_string());
    vm.execute();
}
