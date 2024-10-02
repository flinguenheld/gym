use crate::math_operation::OperationType;
use crate::{math_operation, window};
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

const X_TXT: u16 = 18;
const Y_TXT: u16 = 6;

pub fn run(operation_type: OperationType, nb_terms: u16, min: i16, max: i16) {
    // Init --
    let mut success: u16 = 0;
    let mut fails: u16 = 0;
    let mut warning = false;

    let title = match operation_type {
        OperationType::Addition => "Maths - Addition",
        OperationType::Subtraction => "Maths - Subtraction",
        OperationType::Multiplication => "Maths - Multiplication",
        OperationType::All => "Maths - All",
    };

    let mut operation =
        math_operation::OperationNEWGEN::new(operation_type, nb_terms, min, max).generate();
    let mut user_input = String::from("");

    // Raw mode mandatory to read key events --
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    update_screen(
        title,
        &operation,
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
                if user_input == operation.result {
                    success += 1;
                    warning = false;
                    operation = operation.generate();
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
            title,
            &operation,
            success,
            fails,
            warning,
            &user_input,
            &mut stdout,
        );
    }
}

fn update_screen(
    title: &str,
    operation: &math_operation::OperationNEWGEN,
    success: u16,
    fails: u16,
    warning: bool,
    user_input: &String,
    stdout: &mut RawTerminal<Stdout>,
) {
    window::print_window(title, success, fails, warning, 3);
    print!(
        "{} {} = {}",
        termion::cursor::Goto(X_TXT, Y_TXT),
        operation.to_string,
        user_input
    );

    stdout.flush().unwrap();
}
