mod base;
mod keyboard;
mod math;
mod math_operation;
mod vocabulary;
mod window;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    long_about = "Practice makes perfect!
Gym allows you to train your maths, keyboard and vocabulary skills in your terminal.
See more information here: https://github.com/flinguenheld/gym

ex:     gym -m AS4      -> Maths: Additions and Subtrations with 2 to 4 terms
        gym -k LCN15    -> Keyboard: Letters, Caps, Numbers with 15 characters",
    after_help = "Ctrl+C or ESC to quit\nCtrl+P to pass the current question"
)]
struct Args {
    #[arg(
        short,
        long,
        help = "(A)ddition (S)ubtraction (M)ultiplication (D)ivision (2..) number of terms [default: 2]
(Answer can be another operation: 2 + 2 = 4 / 2 + 1 * 2)"
    )]
    maths: Option<String>,
    #[arg(
        long,
        default_value_t = 2,
        allow_negative_numbers = true,
        help = "min value for maths (can be negative)"
    )]
    min: i32,
    #[arg(
        long,
        default_value_t = 9,
        allow_negative_numbers = true,
        help = "max value for maths (can be negative)"
    )]
    max: i32,
    #[arg(short, long, help = "(B)inary (D)ecimal (H)exadecimal (100..) maxi")]
    base: Option<String>,
    #[arg(
        short,
        long,
        help = "Press Tab to display a hint, requires the file 'vocabulary.txt' in the same folder than gym"
    )]
    vocabulary: bool,
    #[arg(
        short,
        long,
        help = "(L)etters (C)aps (N)umbers (S)ymbols (1..) nb characters [default: 3]"
    )]
    keyboard: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(m) = args.maths {
        math::run(m.to_uppercase(), args.min, args.max);
    } else if let Some(k) = args.keyboard {
        keyboard::run(&k.to_uppercase());
    } else if args.vocabulary {
        vocabulary::run();
    } else if let Some(mode) = args.base {
        base::run(mode);
    } else {
        println!("No option given, see gym -h");
    }
}
