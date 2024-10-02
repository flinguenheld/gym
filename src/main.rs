mod keyboard;
mod math;
mod math_operation;
mod vocabulary;
mod window;
use clap::Parser;
use math_operation::OperationType;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None,after_help = "  -> Ctrl+C or ESC to quit")]
struct Args {
    #[arg(long)]
    maths: bool,
    #[arg(short, long)]
    multiplication: bool,
    #[arg(short, long)]
    addition: bool,
    #[arg(short, long)]
    subtraction: bool,
    #[arg(long, default_value_t = 2, help = "Number of terms for maths (2-..)")]
    terms: u16,
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

    #[arg(short, long, help = "Press Tab for help, requires 'vocabulary.txt'")]
    vocabulary: bool,

    #[arg(
        short,
        long,
        required = false,
        help = "Options: (L)etters (C)aps (N)umbers (S)ymbols (1-...) nb characters"
    )]
    keyboard: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(a) = args.keyboard {
        keyboard::run(&a.to_uppercase());
    } else if args.vocabulary {
        vocabulary::run();
    } else if args.maths {
        math::run(OperationType::All, args.terms, args.min, args.max);
    } else if args.multiplication {
        math::run(
            OperationType::Multiplication,
            args.terms,
            args.min,
            args.max,
        );
    } else if args.addition {
        math::run(OperationType::Addition, args.terms, args.min, args.max);
    // } else if args.subtraction {
    } else {
        math::run(OperationType::Subtraction, args.terms, args.min, args.max);
    }
}
