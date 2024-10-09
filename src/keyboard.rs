use crate::window::{self, Window};
use rand::Rng;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub fn run(options: &str) {
    let mut nb: u32 = options
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0_u32, |acc, d| {
            acc.checked_mul(10).unwrap_or(0) + d.to_digit(10).unwrap_or(0)
        });

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

    if !char_list.is_empty() {
        let mut window = Window::new("Keyboard".to_string());
        let mut current_value = new_value(&char_list, nb);
        let mut user_input = String::from("");

        // Raw mode mandatory to read key events --
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        update_screen(&window, &current_value, &user_input, &mut stdout);

        // Game loop --
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc | Key::Ctrl('c') => {
                    print!("{}", termion::clear::All);
                    break;
                }
                Key::Char('\n') => {
                    if current_value == user_input {
                        window.success += 1;
                        window.icon = window::Icon::None;
                        current_value = new_value(&char_list, nb);
                    } else {
                        window.fails += 1;
                        window.icon = window::Icon::Cross;
                    }

                    user_input.clear();
                }
                Key::Char(c) => {
                    user_input.push(c);
                }
                Key::Backspace => {
                    user_input.pop();
                }
                Key::Ctrl('u') => {
                    user_input.clear();
                }
                Key::Ctrl('p') | Key::Ctrl('P') => {
                    window.fails += 1;
                    window.icon = window::Icon::None;
                    current_value = new_value(&char_list, nb);
                }
                _ => {}
            }

            update_screen(&window, &current_value, &user_input, &mut stdout);
        }

        window.exit();
    } else {
        window::exit_with_error("Wrong options.");
    }
}

// --
fn new_value(char_list: &str, nb_chars: u32) -> String {
    let mut val = String::from("");

    for _ in 0..nb_chars {
        if let Some(c) = char_list
            .chars()
            .nth(rand::thread_rng().gen_range(0..char_list.len()))
        {
            val.push(c);
        }
    }
    val
}

// --
fn update_screen(
    window: &Window,
    current_value: &str,
    user_input: &str,
    stdout: &mut RawTerminal<Stdout>,
) {
    window.print(window::format(
        &format!("{} -> {}", current_value, user_input),
        45,
        false,
    ));
    stdout.flush().unwrap();
}
