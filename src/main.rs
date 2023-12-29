use std::str::Chars;

use clap::Parser;

#[derive(Parser)]
#[command(author = "ydolev", version = "0.1.0", about = "A minimalist C compiler written in Rust", long_about = None)]
struct Cli {
    expression: String,
}

const EOF_CHAR: char = '\0';

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!(".text");
    println!();
    println!(".global _main");
    println!("_main:");

    let mut chars = cli.expression.chars();

    println!("  mov x0, #{}", lex_number(&cli.expression, &mut chars)?);

    while !is_eof(&chars) {
        let at = peek_first(&chars);
        advance(&mut chars);

        match at {
            '+' => println!(
                "  add x0, x0, #{}",
                lex_number(&cli.expression, &mut chars)?
            ),
            '-' => println!(
                "  sub x0, x0, #{}",
                lex_number(&cli.expression, &mut chars)?
            ),
            _ => return Err(rustyc::Error::SyntaxError(at).into()),
        }
    }

    println!("  ret");

    Ok(())
}

fn lex_number(source: &str, chars: &mut Chars) -> rustyc::Result<u64> {
    let start_index = index(source, chars);
    while peek_first(chars).is_ascii_digit() {
        advance(chars);
    }

    let number_source = &source[start_index..index(source, chars)];
    number_source
        .parse()
        .map_err(|e| rustyc::Error::ParseNumber(e, number_source.to_owned()))
}

fn peek_first(chars: &Chars) -> char {
    peek_nth(chars, 0)
}

fn peek_nth(chars: &Chars, n: usize) -> char {
    chars.clone().nth(n).unwrap_or(EOF_CHAR)
}

fn index(source: &str, chars: &Chars) -> usize {
    source.len() - chars.as_str().len()
}

fn is_eof(chars: &Chars) -> bool {
    EOF_CHAR == peek_first(chars)
}

fn advance(chars: &mut Chars) {
    chars.next();
}
