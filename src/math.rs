use crate::{math_operation, window};
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

const X_TXT: u16 = 18;
const Y_TXT: u16 = 6;

pub fn run(options: String, min: i32, max: i32) {
    let mut nb_terms = options
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0_u32, |acc, d| acc * 10 + d.to_digit(10).unwrap_or(0));

    if nb_terms == 0 {
        nb_terms = 2;
    }

    // Init --
    let mut success: u16 = 0;
    let mut fails: u16 = 0;
    let mut warning = false;

    let mut operation = math_operation::Operation::new(
        options
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect(),
        nb_terms,
        min,
        max,
    );
    operation.generate();
    let mut user_input = String::from("");

    // Raw mode mandatory to read key events --
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    update_screen(
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
                    operation.generate();
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
            Key::Char('p') | Key::Char('P') => {
                fails += 1;
                operation.generate();
                user_input.clear();
            }
            _ => {}
        }

        update_screen(
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
    operation: &math_operation::Operation,
    success: u16,
    fails: u16,
    warning: bool,
    user_input: &String,
    stdout: &mut RawTerminal<Stdout>,
) {
    window::print_window("Maths", success, fails, warning, 3);
    print!(
        "{} {} = {}",
        termion::cursor::Goto(X_TXT, Y_TXT),
        operation.to_string,
        user_input
    );

    stdout.flush().unwrap();
}
