use crate::math_operation::convert_and_resolve;
use crate::window::Window;
use crate::{math_operation, window};
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub fn run(options: String, min: i64, max: i64, max_cons_div: bool) {
    if let Some(mut operation) = math_operation::Operation::new(
        options
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect(),
        options
            .chars()
            .filter(|c| c.is_ascii_digit())
            .fold(0_u32, |acc, d| {
                acc.checked_mul(10).unwrap_or(0) + d.to_digit(10).unwrap_or(0)
            }),
        min,
        max,
        max_cons_div,
    ) {
        // Raw mode mandatory to read key events --
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        let mut window = Window::new("Maths".to_string());
        let mut user_input = String::from("");
        let mut answer_given = false;
        new_operation(&mut operation);

        update_screen(&operation, &window, user_input.as_str(), &mut stdout);

        // Game loop --
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc | Key::Ctrl('c') => {
                    print!("{}", termion::clear::All);
                    break;
                }
                Key::Char('\n') => {
                    if !user_input.is_empty() {
                        if user_input == operation.result {
                            if !answer_given {
                                window.success += 1;
                            }
                            window.icon = window::Icon::None;
                            new_operation(&mut operation);
                        } else if let Ok(user_operation_result) = convert_and_resolve(&user_input) {
                            if user_operation_result == operation.result {
                                operation.to_string = math_operation::clean_operation(&user_input);
                                window.icon = window::Icon::Loop;
                            } else {
                                window.fails += 1;
                                window.icon = window::Icon::Cross;
                            }
                        } else {
                            window.fails += 1;
                            window.icon = window::Icon::Cross;
                        }

                        answer_given = false;
                        user_input.clear();
                    }
                }
                Key::Char(c)
                    if c.is_ascii_digit() || c == '+' || c == '-' || c == '*' || c == '/' =>
                {
                    user_input.push(c);
                }
                Key::Backspace => {
                    user_input.pop();
                }
                Key::Ctrl('a') | Key::Ctrl('A') => {
                    window.icon = window::Icon::Gift;
                    user_input = operation.result.clone();
                    answer_given = true;
                }
                Key::Ctrl('p') | Key::Ctrl('P') => {
                    window.fails += 1;
                    new_operation(&mut operation);
                    user_input.clear();
                    window.icon = window::Icon::None;
                }
                _ => {}
            }

            update_screen(&operation, &window, user_input.as_str(), &mut stdout);
        }

        window.exit();
    } else {
        window::exit_with_error("Wrong options.");
    }
}

fn new_operation(operation: &mut math_operation::Operation) {
    if operation.generate().is_err() {
        window::exit_with_error("Overflow! Please reduce the options.")
    }
}

fn update_screen(
    operation: &math_operation::Operation,
    window: &Window,
    user_input: &str,
    stdout: &mut RawTerminal<Stdout>,
) {
    window.print(format!(
        "{} = {}",
        window::format(&operation.to_string, 28, true),
        window::format(user_input, 23, false),
    ));
    stdout.flush().unwrap();
}
