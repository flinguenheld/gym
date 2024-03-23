use clap::Parser;
use rand::Rng;
use std::io;
use std::io::Write;

const X: u16 = 17;
const Y: u16 = 6;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    multiplication: bool,
    #[arg(short, long)]
    addition: bool,
    #[arg(short, long)]
    substraction: bool,

    #[arg(long, default_value_t = 2)]
    min: u16,
    #[arg(long, default_value_t = 9)]
    max: u16,
}

fn main() {
    let mut args = Args::parse();
    if args.min > args.max {
        args.max = args.min + 5;
    }

    let mut success: u16 = 0;
    let mut fails: u16 = 0;
    let mut warning = false;

    let mut a = 1;
    let mut b = 1;
    let mut result = String::from("");

    loop {
        if !warning {
            a = rand::thread_rng().gen_range(args.min, args.max);
            b = rand::thread_rng().gen_range(args.min, args.max);
        }

        if args.multiplication {
            result = (a * b).to_string();
            print_window("Multiplication", success, fails, warning);
            print!("{} {} * {} = ", termion::cursor::Goto(X, Y), a, b);
        } else if args.addition {
            result = (a + b).to_string();
            print_window("Addition", success, fails, warning);
            print!("{} {} + {} = ", termion::cursor::Goto(X, Y), a, b);
        } else if args.substraction {
            result = (a - b).to_string();
            print_window("Substraction", success, fails, warning);
            print!("{} {} - {} = ", termion::cursor::Goto(X, Y), a, b);
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

fn print_window(title: &str, success: u16, fails: u16, warning: bool) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 2));
    println!("    ┌───────────────────────────┬────────┬────────┐");
    println!("    │                           │        │        │");
    println!("    ├───────────────────────────┴────────┴────────┤");
    println!("    │                                             │");
    println!("    │                                             │");
    println!("    │                                             │");
    println!("    └─────────────────────────────────────────────┘");

    print!("{}{}", termion::cursor::Goto(7, 3), title);

    print!("{} ✅ {}", termion::cursor::Goto(35, 3), success);
    print!("{} ❌ {}", termion::cursor::Goto(44, 3), fails);

    if warning {
        print!("{}❌", termion::cursor::Goto(13, 6));
    }
}
