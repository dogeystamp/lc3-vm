//////////////////////////////
////// terminal input/output
//////////////////////////////

extern crate termios;
use termios::*;

extern crate libc;
use libc::STDIN_FILENO;

extern crate ctrlc;

use std::io;
use std::io::Read;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

////////////////
// keyboard I/O interface
////////////////

pub trait KeyboardIO {
    /// Poll stdin for a keypress
    fn get_key(&mut self) -> Option<u8>;
    /// Peek to see if there is a key
    fn check_key(&mut self) -> bool;
}

pub struct TerminalIO {
    stdin_channel: Receiver<u8>,
    char: Option<u8>,
}

impl TerminalIO {
    pub fn new() -> TerminalIO {
        setup_termios();
        TerminalIO {
            stdin_channel: Self::spawn_stdin_channel(),
            char: None,
        }
    }

    fn spawn_stdin_channel() -> Receiver<u8> {
        // https://stackoverflow.com/questions/30012995
        let (tx, rx) = mpsc::channel::<u8>();
        let mut buffer: [u8; 1] = [0];
        thread::spawn(move || loop {
            let _ = io::stdin().lock().read_exact(&mut buffer);
            let _ = tx.send(buffer[0]);
        });
        rx
    }
}

impl Drop for TerminalIO {
    fn drop(&mut self) {
        restore_terminal();
    }
}

impl KeyboardIO for TerminalIO {
    fn get_key(&mut self) -> Option<u8> {
        self.char
    }

    fn check_key(&mut self) -> bool {
        match self.stdin_channel.try_recv() {
            Ok(key) => { self.char = Some(key); true },
            Err(mpsc::TryRecvError::Empty) => false,
            Err(mpsc::TryRecvError::Disconnected) => panic!("terminal keyboard stream broke"),
        }
    }
}

////////////////
// termios stuff
////////////////

/// Configure raw input (see termios(3) man-page)
fn setup_termios() {
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
