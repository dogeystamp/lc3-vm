//////////////////////////////
////// driver code
//////////////////////////////

mod vm;
use crate::vm::{terminal_io, VM};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enables debug printing to stderr (which can be separately piped to a file).
    #[arg(long)]
    debug: bool,

    /// Program file
    program: Option<String>,
}

fn main() {
    let cli = Args::parse();

    let mut term = terminal_io::TerminalIO::new();

    let mut vm = VM::new(&mut term);
    vm.set_debugging(cli.debug);

    vm.read_program(&cli.program.expect("No program file given"));
    vm.execute();
}
