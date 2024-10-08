use crate::window::{self, Window};

use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

const AVAILABLE_OPT: [char; 3] = ['B', 'D', 'H'];

pub fn run(options: String) {
    let mut maxi: u32 = options
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0_u32, |acc, d| {
            acc.checked_mul(10).unwrap_or(0) + d.to_digit(10).unwrap_or(0)
        });

    if maxi < 10 {
        maxi = 10;
    }

    let options: Vec<char> = options
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    let base_question = options.first().unwrap_or(&'\0').to_ascii_uppercase();
    let base_answer = options.last().unwrap_or(&'\0').to_ascii_uppercase();

    if AVAILABLE_OPT.contains(&base_question)
        && AVAILABLE_OPT.contains(&base_answer)
        && base_question != base_answer
    {
        let mut window = Window::new(format!(
            "{} to {}",
            option_to_string(base_question),
            option_to_string(base_answer)
        ));

        let (mut question, mut answer) = new_value(base_question, base_answer, maxi);
        let mut user_input = String::new();
        let mut answer_given = false;

        // Raw mode mandatory to read key events --
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        update_screen(&window, question.as_str(), "".to_string(), &mut stdout);

        // Game loop --
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc | Key::Ctrl('c') => {
                    print!("{}", termion::clear::All);
                    break;
                }
                Key::Char('\n') => {
                    if answer == user_input.to_uppercase() {
                        if !answer_given {
                            window.success += 1;
                        }
                        window.icon = window::Icon::None;
                        (question, answer) = new_value(base_question, base_answer, maxi);
                    } else {
                        window.fails += 1;
                        window.icon = window::Icon::Cross;
                    }

                    user_input.clear();
                    answer_given = false;
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
                    window.icon = window::Icon::Gift;
                    user_input = answer.clone();
                    answer_given = true;
                }
                Key::Ctrl('p') | Key::Ctrl('P') => {
                    window.fails += 1;
                    window.icon = window::Icon::Cross;
                    (question, answer) = new_value(base_question, base_answer, maxi);
                }
                _ => {}
            }

            update_screen(
                &window,
                question.as_str(),
                format_to_display(base_answer, user_input.as_str()),
                &mut stdout,
            );
        }
    } else {
        println!("\x1b[91mError: \x1b[0m Wrong options, see gym -h");
    }
}

/// Give the full base text
fn option_to_string(base: char) -> &'static str {
    match base {
        'B' => "binary",
        'D' => "decimal",
        _ => "hexadecimal",
    }
}

/// Generate a new value then return the formated question and the unformated answer
fn new_value(base_question: char, base_answer: char, maxi: u32) -> (String, String) {
    let val = rand::thread_rng().gen_range(1..=maxi);

    let question = from_int(base_question, val);
    let answer = from_int(base_answer, val);

    (format_to_display(base_question, question.as_str()), answer)
}

fn from_int(base: char, new_value: u32) -> String {
    match base {
        'B' => format!("{:b}", new_value),
        'H' => format!("{:X}", new_value),
        _ => format!("{}", new_value),
    }
}

/// Add whitespaces to help reading
fn format_to_display(base: char, value: &str) -> String {
    let nb_per_group = match base {
        'B' => 4,
        'H' => 4,
        _ => 3,
    };

    let mut output = VecDeque::new();
    for (i, c) in value.chars().rev().enumerate() {
        if i > 0 && i % nb_per_group == 0 {
            output.push_front(' ');
        }
        output.push_front(c);
    }
    output.iter().collect()
}

// --
fn update_screen(
    window: &Window,
    question: &str,
    answer: String,
    stdout: &mut RawTerminal<Stdout>,
) {
    window.print(format!(
        "{} -> {}",
        window::format(question, 28, true),
        window::format(&answer, 23, true),
    ));
    stdout.flush().unwrap();
}
