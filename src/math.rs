use crate::window;

use rand::Rng;
use std::io;
use std::io::Write;

const X_TXT: u16 = 18;
const Y_TXT: u16 = 6;

#[derive(PartialEq)]
pub enum Operation {
    Addition,
    Multiplication,
    Substraction,
}

pub fn run(operation_type: Operation, min: u16, mut max: u16) {
    if min > max {
        max = min + 5;
    }

    let mut success: u16 = 0;
    let mut fails: u16 = 0;
    let mut warning = false;

    let mut a = 1;
    let mut b = 1;
    let mut result = String::from("");

    loop {
        if !warning {
            a = rand::thread_rng().gen_range(min, max + 1);
            b = rand::thread_rng().gen_range(min, max + 1);
        }

        if operation_type == Operation::Multiplication {
            result = (a * b).to_string();
            window::print_window("Multiplication", success, fails, warning);
            print!("{} {} * {} = ", termion::cursor::Goto(X_TXT, Y_TXT), a, b);
        } else if operation_type == Operation::Addition {
            result = (a + b).to_string();
            window::print_window("Addition", success, fails, warning);
            print!("{} {} + {} = ", termion::cursor::Goto(X_TXT, Y_TXT), a, b);
        } else if operation_type == Operation::Substraction {
            result = (a - b).to_string();
            window::print_window("Substraction", success, fails, warning);
            print!("{} {} - {} = ", termion::cursor::Goto(X_TXT, Y_TXT), a, b);
        }

        io::stdout().flush().unwrap();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Error");

        if answer.trim() == result {
            success += 1;
            warning = false;
        } else {
            fails += 1;
            warning = true;
        }
    }
}
