//////////////////////////////
////// terminal input/output
//////////////////////////////

extern crate termios;
use termios::*;

extern crate libc;
use libc::STDIN_FILENO;

extern crate ctrlc;

/// Configure raw input (see termios(3) man-page)
pub fn setup_terminal() {
    let mut term: Termios = Termios::from_fd(STDIN_FILENO).unwrap();
    // ICANON (canonical) is line-by-line input (i.e. press enter to send)
    // ECHO is showing the characters you type
    // what this means is that LC-3 will receive characters immediately and without displaying them
    term.c_lflag &= !(ICANON | ECHO);
    // TCSANOW: "the change occurs immediately"
    tcsetattr(STDIN_FILENO, TCSANOW, &term).unwrap();

    // when leaving the program we want to be polite and undo the above changes
    ctrlc::set_handler(|| {
        restore_terminal();
        // typical CTRL-C exit code
        std::process::exit(130);
    })
    .expect("Failed to set CTRL-C handler");
}

/// Restore terminal to initial state
fn restore_terminal() {
    // Ideally we'd store the original state but I was too lazy
    let mut term: Termios = Termios::from_fd(STDIN_FILENO).unwrap();
    term.c_lflag |= ICANON | ECHO;
    tcsetattr(STDIN_FILENO, TCSANOW, &term).unwrap()
}
