use crate::window;

use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const X_TXT: u16 = 15;
const Y_TXT: u16 = 6;

#[derive(Debug)]
struct Word {
    value: String,
    synonym: Vec<String>,
}

pub fn run() {
    // Read file and create the word list
    if let Ok(file_content) = fs::read_to_string("vocabulary.txt") {
        let mut words: Vec<Word> = Vec::new();
        file_content.split('\n').for_each(|line| {
            line.split(';').for_each(|field| {
                if let Some((a, b)) = field.split_once('=') {
                    words.push(Word {
                        value: a.to_string(),
                        synonym: b.split(',').map(|val| val.to_string()).collect(),
                    });
                }
            })
        });

        // --
        let mut success: u16 = 0;
        let mut fails: u16 = 0;
        let mut warning = false;

        if let Some(mut current_word) = words.first() {
            loop {
                if !warning {
                    current_word = &words[rand::thread_rng().gen_range(0, words.len())];
                }

                window::print_window("English", success, fails, warning);
                print!(
                    "{}{} = ",
                    termion::cursor::Goto(X_TXT, Y_TXT),
                    &current_word.value
                );

                io::stdout().flush().unwrap();

                let mut answer = String::new();
                io::stdin().read_line(&mut answer).expect("Error");

                if current_word.synonym.contains(&answer.trim().to_string()) {
                    success += 1;
                    warning = false;
                } else {
                    fails += 1;
                    warning = true;
                }
            }
        } else {
            println!();
            println!("   The file 'vocabulary.txt' is empty or corrupted, please recreate it with this structure:");
            println!("   word=first_synonym,second_synonym,...");
            println!("   word=first_synonym,second_synonym,...");
            println!("   ...");
            println!();
        }
    } else {
        println!();
        println!("   Error file, please create a file 'vocabulary.txt' in the same folder with this structure:");
        println!("   word=first_synonym,second_synonym,...");
        println!("   word=first_synonym,second_synonym,...");
        println!("   ...");
        println!();
    }
}
