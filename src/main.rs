use clap::Parser;

#[derive(Parser)]
#[command(author = "ydolev", version = "0.1.0", about = "A minimalist C compiler written in Rust", long_about = None)]
struct Cli {
    number: i64,
}

fn main() {
    let cli = Cli::parse();

    println!(".text");
    println!();
    println!(".global _main");
    println!("_main:");
    println!("  mov x0, #{}", cli.number);
    println!("  ret");
}
