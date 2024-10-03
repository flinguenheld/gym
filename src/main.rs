mod keyboard;
mod math;
mod math_operation;
mod vocabulary;
mod window;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None,after_help = "  -> Ctrl+C or ESC to quit")]
struct Args {
    #[arg(
        short,
        long,
        required = false,
        help = "Options: (A)ddition (S)ubtraction (M)ultiplication (2..) number of terms"
    )]
    maths: Option<String>,
    #[arg(
        long,
        default_value_t = 2,
        help = "min value for maths (can be negative)"
    )]
    min: i32,
    #[arg(
        long,
        default_value_t = 9,
        help = "max value for maths (can be negative)"
    )]
    max: i32,

    #[arg(short, long, help = "Press Tab for help, requires 'vocabulary.txt'")]
    vocabulary: bool,

    #[arg(
        short,
        long,
        required = false,
        help = "Options: (L)etters (C)aps (N)umbers (S)ymbols (1..) nb characters"
    )]
    keyboard: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(m) = args.maths {
        math::run(m.to_uppercase(), args.min, args.max);
    } else if let Some(a) = args.keyboard {
        keyboard::run(&a.to_uppercase());
    } else if args.vocabulary {
        vocabulary::run();
    }
}
