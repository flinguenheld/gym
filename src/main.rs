mod keyboard;
mod math;
mod vocabulary;
mod window;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None,after_help = "  -> Ctrl+C or ESC to quit")]
struct Args {
    #[arg(short, long)]
    multiplication: bool,
    #[arg(short, long)]
    addition: bool,
    #[arg(short, long)]
    substraction: bool,

    #[arg(short, long, help = "Press Tab for help, requires 'vocabulary.txt'")]
    vocabulary: bool,

    #[arg(
        short,
        long,
        required = false,
        help = "Options: (L)etters (C)aps (N)umbers (S)ymbols (0-9...)nb characters"
    )]
    keyboard: Option<String>,

    #[arg(
        long,
        default_value_t = 2,
        help = "min value for maths (can be negative)"
    )]
    min: i16,
    #[arg(
        long,
        default_value_t = 9,
        help = "max value for maths (can be negative)"
    )]
    max: i16,
}

fn main() {
    let args = Args::parse();

    if let Some(a) = args.keyboard {
        keyboard::run(&a.to_uppercase());
    } else if args.vocabulary {
        vocabulary::run();
    } else if args.addition {
        math::run(math::Operation::Addition, args.min, args.max);
    } else if args.substraction {
        math::run(math::Operation::Substraction, args.min, args.max);
    } else {
        math::run(math::Operation::Multiplication, args.min, args.max);
    }
}
