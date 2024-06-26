use crate::window;

use rand::Rng;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

const X_TXT: u16 = 18;
const Y_TXT: u16 = 6;

pub fn run(options: &str) {
    // Init --
    let mut nb: u32 = options
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap_or(0));

    if nb == 0 {
        nb = 3;
    }

    let mut char_list = String::from("");

    if options.contains('L') {
        char_list.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if options.contains('C') {
        char_list.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if options.contains('N') {
        char_list.push_str("0123456789");
    }
    if options.contains('S') {
        char_list.push_str("$^[]&|~!?{}\"\\.,()*_-:;<>/'`@%#+=");
    }

    if char_list.is_empty() {
        println!("\x1b[91mError: \x1b[0m Wrong options, try gym -h");
        return;
    }

    let mut success: u16 = 0;
    let mut fails: u16 = 0;
    let mut warning = false;
    let mut current_value = new_value(&char_list, nb);
    let mut user_input = String::from("");

    // Raw mode mandatory to read key events --
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    update_screen(
        success,
        fails,
        warning,
        &current_value,
        &user_input,
        &mut stdout,
    );

    // Game loop --
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc | Key::Ctrl('c') => {
                print!("{}", termion::clear::All);
                break;
            }
            Key::Char('\n') => {
                if current_value == user_input {
                    success += 1;
                    warning = false;
                    current_value = new_value(&char_list, nb);
                } else {
                    fails += 1;
                    warning = true;
                }

                user_input.clear();
            }
            Key::Char(c) => {
                user_input.push(c);
            }
            Key::Backspace => {
                user_input.pop();
            }
            _ => {}
        }

        update_screen(
            success,
            fails,
            warning,
            &current_value,
            &user_input,
            &mut stdout,
        );
    }
}

// --
fn new_value(char_list: &str, nb_chars: u32) -> String {
    let mut val = String::from("");

    for _ in 0..nb_chars {
        if let Some(c) = char_list
            .chars()
            .nth(rand::thread_rng().gen_range(0, char_list.len()))
        {
            val.push(c);
        }
    }
    val
}

// --
fn update_screen(
    success: u16,
    fails: u16,
    warning: bool,
    current_value: &str,
    user_input: &str,
    stdout: &mut RawTerminal<Stdout>,
) {
    window::print_window("Keyboard", success, fails, warning, 3);

    print!(
        "{} {} -> {}",
        termion::cursor::Goto(X_TXT, Y_TXT),
        current_value,
        user_input
    );

    stdout.flush().unwrap();
}
