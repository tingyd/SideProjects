
    use clap::Parser;
use rand::prelude::*;

/// Simple CLI password generator
#[derive(Parser, Debug)]
#[command(author, version, about = "Generate random passwords", long_about = None)]
struct Args {
    /// Length of each password
    #[arg(short = 'l', long = "length", default_value_t = 16)]
    length: usize,

    /// How many passwords to generate
    #[arg(short = 'n', long = "count", default_value_t = 1)]
    count: usize,

    /// Include lowercase letters
    #[arg(long = "lower", default_value_t = true)]
    include_lower: bool,

    /// Include uppercase letters
    #[arg(long = "upper", default_value_t = true)]
    include_upper: bool,

    /// Include digits
    #[arg(long = "digits", default_value_t = true)]
    include_digits: bool,

    /// Include symbols (punctuation)
    #[arg(long = "symbols", default_value_t = true)]
    include_symbols: bool,

    /// Avoid ambiguous characters like 'l', 'I', '1', 'O', '0'
    #[arg(long = "no-ambiguous", default_value_t = false)]
    no_ambiguous: bool,
}

fn main() {
    let args = Args::parse();

    // Build character set
    let mut charset: Vec<char> = Vec::new();

    const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGITS: &str = "0123456789";
    const SYMBOLS: &str = r#"!@#$%^&*()-_=+[]{};:,.<>/?\"'#"#;

    if args.include_lower {
        charset.extend(LOWER.chars());
    }
    if args.include_upper {
        charset.extend(UPPER.chars());
    }
    if args.include_digits {
        charset.extend(DIGITS.chars());
    }
    if args.include_symbols {
        charset.extend(SYMBOLS.chars());
    }

    if args.no_ambiguous {
        // Remove common ambiguous characters
        let ambiguous = ['l', 'I', '1', 'O', '0'];
        charset.retain(|c| !ambiguous.contains(c));
    }

    if charset.is_empty() {
        eprintln!("Error: character set is empty. Enable at least one of --lower/--upper/--digits/--symbols.");
        std::process::exit(1);
    }

    let mut rng = thread_rng();

    for _ in 0..args.count {
        let password: String = (0..args.length)
            .map(|_| {
                // choose returns Option<&T>, unwrap is safe because charset is non-empty
                *charset.choose(&mut rng).unwrap()
            })
            .collect();

        println!("{}", password);
    }
}
