use std::slice::Iter;

use clap::Parser;

use rustyc::{Lexer, Token};

#[derive(Parser)]
#[command(author = "ydolev", version = "0.1.0", about = "A minimalist C compiler written in Rust", long_about = None)]
struct Cli {
    expression: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let source = Rc::new(cli.expression);

    let 
    let lexer = Lexer::new(source.as_str());
    let tokens = lexer.lex()?;
    let mut tokens_iterator = tokens.iter();

    println!(".text");
    println!();
    println!(".global _main");
    println!("_main:");

    println!(
        "  mov x0, #{}",
        expect_number(expect_token(&mut tokens_iterator)?)?
    );

    while let Some(token) = tokens_iterator.next() {
        let punctuator = expect_punctuator(token)?;
        match punctuator {
            '+' => {
                println!(
                    "  add x0, x0, #{}",
                    expect_number(expect_token(&mut tokens_iterator)?)?
                );
            }
            '-' => {
                println!(
                    "  sub x0, x0, #{}",
                    expect_number(expect_token(&mut tokens_iterator)?)?
                );
            }
            _ => {}
        }
    }

    println!("  ret");

    Ok(())
}

fn expect_token<'a>(tokens: &'a mut Iter<Token>) -> rustyc::Result<&'a Token> {
    tokens.next().ok_or(rustyc::Error::UnexpectedEof)
}

fn expect_number(token: &Token) -> rustyc::Result<u64> {
    match token {
        Token::Number(value) => Ok(*value),
        _ => Err(rustyc::Error::UnexpectedToken(
            token.clone(),
            String::from("number"),
        )),
    }
}

fn expect_punctuator(token: &Token) -> rustyc::Result<char> {
    match token {
        Token::Punctuator(character) => Ok(*character),
        _ => Err(rustyc::Error::UnexpectedToken(
            token.clone(),
            String::from("punctuator"),
        )),
    }
}
