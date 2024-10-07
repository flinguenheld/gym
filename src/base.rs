use crate::window;

use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

const AVAILABLE_OPT: [char; 3] = ['B', 'D', 'H'];

pub fn run(options: String) {
    // Init --
    let mut maxi: u32 = options
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap_or(0));

    if maxi < 10 {
        maxi = 10;
    }

    let opt: Vec<char> = options
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    let base_from = opt.first().unwrap_or(&'\0').to_ascii_uppercase();
    let base_to = opt.last().unwrap_or(&'\0').to_ascii_uppercase();

    let title = format!(
        "{} to {}",
        match base_from {
            'B' => "binary",
            'D' => "decimal",
            _ => "hexadecimal",
        },
        match base_to {
            'B' => "binary",
            'D' => "decimal",
            _ => "hexadecimal",
        }
    );

    if AVAILABLE_OPT.contains(&base_from)
        && AVAILABLE_OPT.contains(&base_to)
        && base_from != base_to
    {
        // --
        let mut success: u16 = 0;
        let mut fails: u16 = 0;
        let mut warning = "";
        let (mut current_value, mut current_answer) = new_value(maxi, base_from, base_to);
        let mut user_input = String::from("");

        // Raw mode mandatory to read key events --
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        update_screen(
            success,
            fails,
            warning,
            title.as_str(),
            &current_value,
            (base_from, base_to),
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
                    if current_answer == user_input {
                        success += 1;
                        warning = "";
                        (current_value, current_answer) = new_value(maxi, base_from, base_to);
                    } else {
                        fails += 1;
                        warning = "❌";
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
                Key::Ctrl('a') | Key::Ctrl('A') => {
                    fails += 1;
                    user_input = current_answer.clone();
                    // TODO: Add a bool to skip the next enter
                }
                Key::Ctrl('p') | Key::Ctrl('P') => {
                    fails += 1;
                    warning = "";
                    (current_value, current_answer) = new_value(maxi, base_from, base_to);
                }
                _ => {}
            }

            update_screen(
                success,
                fails,
                warning,
                title.as_str(),
                &current_value,
                (base_from, base_to),
                &user_input,
                &mut stdout,
            );
        }
    } else {
        println!("Incorrect options, see gym -h");
    }
}

// --
fn new_value(maxi: u32, from_type: char, to_type: char) -> (String, String) {
    let val = rand::thread_rng().gen_range(1..=maxi);
    (convert(from_type, val), convert(to_type, val))
}
fn convert(to: char, value: u32) -> String {
    match to {
        'B' => format!("{:b}", value),
        'H' => format!("{:X}", value),
        _ => format!("{}", value),
    }
}

// --
fn update_screen(
    success: u16,
    fails: u16,
    warning: &str,
    title: &str,
    current_value: &str,
    formats: (char, char),
    user_input: &str,
    stdout: &mut RawTerminal<Stdout>,
) {
    window::print_window(
        window::format(
            format!(
                "{} -> {}",
                format(current_value, formats.0),
                format(user_input, formats.1)
            ),
            45,
            false,
        ),
        title,
        success,
        fails,
        warning,
    );

    stdout.flush().unwrap();
}

/// Add spaces according to the base
fn format(txt: &str, base: char) -> String {
    let nb_per_group = match base {
        'D' => 3,
        _ => 4,
    };

    let mut output = VecDeque::new();
    for (i, c) in txt.chars().rev().enumerate() {
        if i > 0 && i % nb_per_group == 0 {
            output.push_front(' ');
        }
        output.push_front(c);
    }
    output.iter().collect()
}
