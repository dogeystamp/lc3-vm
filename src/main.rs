/*

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation; either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see https://www.gnu.org/licenses/.

© 2024 dogeystamp <dogeystamp@disroot.org>
*/

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
