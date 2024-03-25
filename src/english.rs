use crate::window;
use rand::Rng;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

const X_TXT: u16 = 15;
const Y_TXT: u16 = 6;

#[derive(Debug)]
struct Word {
    value: String,
    synonyms: Vec<String>,
}

pub fn run() {
    // Read file and create the word list
    if let Ok(path) = env::current_dir() {
        println!("{}", path.display());
        if let Ok(file_content) = fs::read_to_string(format!("{}/vocabulary.txt", path.display())) {
            let mut words: Vec<Word> = Vec::new();
            file_content.split('\n').for_each(|line| {
                line.split(';').for_each(|field| {
                    if let Some((a, b)) = field.split_once('=') {
                        words.push(Word {
                            value: a.to_string(),
                            synonyms: b.split(',').map(|val| val.to_string()).collect(),
                        });
                    }
                })
            });

            // Raw mode mandatory to read key events --
            let stdin = stdin();
            let mut stdout = stdout().into_raw_mode().unwrap();

            // --
            let mut success: u16 = 0;
            let mut fails: u16 = 0;
            let mut warning = false;

            let mut user_input = String::from("");
            let mut help = String::from("");

            // Get, check & display the first word --
            if let Some(mut current_word) = get_random_word(&words) {
                update_screen(
                    warning,
                    current_word,
                    &user_input,
                    success,
                    fails,
                    &help,
                    &mut stdout,
                );

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Esc | Key::Ctrl('c') => {
                            print!("{}", termion::clear::All);
                            break;
                        }

                        // Tab to display help (all synonyms with stars).
                        Key::Char('\t') => {
                            help = current_word
                                .synonyms
                                .join(" - ")
                                .chars()
                                .map(|c| {
                                    if c != ' '
                                        && c != '-'
                                        && rand::thread_rng().gen_bool(1.0 / 2.0)
                                    {
                                        '*'
                                    } else {
                                        c
                                    }
                                })
                                .collect();

                            user_input.clear();
                        }

                        Key::Char('\n') => {
                            if current_word
                                .synonyms
                                .contains(&user_input.trim().to_string())
                            {
                                success += 1;
                                warning = false;
                                current_word = get_random_word(&words).unwrap();
                            } else {
                                fails += 1;
                                warning = true;
                            }

                            user_input.clear();
                            help.clear();
                        }

                        Key::Backspace => {
                            user_input.pop();
                        }

                        Key::Char(c) => {
                            user_input.push(c);
                        }

                        _ => {}
                    }

                    update_screen(
                        warning,
                        current_word,
                        &user_input,
                        success,
                        fails,
                        &help,
                        &mut stdout,
                    );
                }
            } else {
                println!("\r");
                println!("   The file 'vocabulary.txt' is empty or corrupted, please recreate it with this structure:\r");
                println!("       word=first_synonym,second_synonym,...\r");
                println!("       word=first_synonym,second_synonym,...\r");
                println!("       ...\r");
                println!("\r");
            }
        } else {
            println!("\r");
            println!("   Error file, please create a file 'vocabulary.txt' in the same folder with this structure:\r");
            println!("       word=first_synonym,second_synonym,...\r");
            println!("       word=first_synonym,second_synonym,...\r");
            println!("       ...\r");
            println!("\r");
        }
    } else {
        println!("\r");
        println!("   Gym's folder is unreachable");
        println!("\r");
    }
}

// --
fn get_random_word(words: &[Word]) -> Option<&Word> {
    words.get(rand::thread_rng().gen_range(0, words.len()))
}

fn update_screen(
    warning: bool,
    current_word: &Word,
    current_txt: &String,
    success: u16,
    fails: u16,
    help: &String,
    stdout: &mut RawTerminal<Stdout>,
) {
    if !help.is_empty() {
        window::print_window("English", success, fails, warning, 5);
        print!("{}💡{}", termion::cursor::Goto(X_TXT - 5, Y_TXT + 2), &help);
    } else {
        window::print_window("English", success, fails, warning, 3);
    }

    print!(
        "{}{} = {}",
        termion::cursor::Goto(X_TXT, Y_TXT),
        &current_word.value,
        &current_txt
    );

    stdout.flush().unwrap();
}
