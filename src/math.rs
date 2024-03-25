use crate::window;

use rand::Rng;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

const X_TXT: u16 = 18;
const Y_TXT: u16 = 6;

#[derive(PartialEq)]
pub enum Operation {
    Addition,
    Multiplication,
    Substraction,
}

pub fn run(operation_type: Operation, min: i16, mut max: i16) {
    if min > max {
        max = min + 5;
    }

    // Init --
    let mut success: u16 = 0;
    let mut fails: u16 = 0;
    let mut warning = false;

    let mut ab = get_random_numbers(min, max);
    let mut user_input = String::from("");

    // Raw mode mandatory to read key events --
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    update_screen(
        &operation_type,
        ab,
        success,
        fails,
        warning,
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
                let result = match operation_type {
                    Operation::Multiplication => (ab.0 * ab.1).to_string(),
                    Operation::Addition => (ab.0 + ab.1).to_string(),
                    Operation::Substraction => (ab.0 - ab.1).to_string(),
                };

                if user_input == result {
                    success += 1;
                    warning = false;
                    ab = get_random_numbers(min, max);
                } else {
                    fails += 1;
                    warning = true;
                }

                user_input.clear();
            }

            Key::Char(c) if c.is_ascii_digit() || c == '-' => {
                user_input.push(c);
            }
            Key::Backspace => {
                user_input.pop();
            }
            _ => {}
        }

        update_screen(
            &operation_type,
            ab,
            success,
            fails,
            warning,
            &user_input,
            &mut stdout,
        );
    }
}

// --
fn get_random_numbers(min: i16, max: i16) -> (i16, i16) {
    (
        rand::thread_rng().gen_range(min, max + 1),
        rand::thread_rng().gen_range(min, max + 1),
    )
}

fn update_screen(
    operation_type: &Operation,
    ab: (i16, i16),
    success: u16,
    fails: u16,
    warning: bool,
    user_input: &String,
    stdout: &mut RawTerminal<Stdout>,
) {
    let (title, operator) = match operation_type {
        Operation::Multiplication => ("Multiplication", "x"),
        Operation::Addition => ("Addition", "+"),
        Operation::Substraction => ("Substraction", "-"),
    };

    window::print_window(title, success, fails, warning, 3);
    print!(
        "{} {} {} {} = {}",
        termion::cursor::Goto(X_TXT, Y_TXT),
        ab.0,
        operator,
        ab.1,
        user_input
    );

    stdout.flush().unwrap();
}
