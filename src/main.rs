mod english;
mod math;
mod window;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None,after_help = "  -> Ctrl+c to quit")]
struct Args {
    #[arg(short, long)]
    multiplication: bool,
    #[arg(short, long)]
    addition: bool,
    #[arg(short, long)]
    substraction: bool,

    #[arg(short, long, help = "Press Tab for help, requires 'vocabulary.txt'")]
    english: bool,

    #[arg(long, default_value_t = 2)]
    min: i16,
    #[arg(long, default_value_t = 9)]
    max: i16,
}

fn main() {
    let args = Args::parse();

    if args.multiplication {
        math::run(math::Operation::Multiplication, args.min, args.max);
    } else if args.addition {
        math::run(math::Operation::Addition, args.min, args.max);
    } else if args.substraction {
        math::run(math::Operation::Substraction, args.min, args.max);
    } else {
        english::run();
    }
}
