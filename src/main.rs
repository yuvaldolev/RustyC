use clap::Parser;

#[derive(Parser)]
#[command(author = "ydolev", version = "0.1.0", about = "A minimalist C compiler written in Rust", long_about = None)]
struct Cli {
    number: i32,
}

fn main() {
    let cli = Cli::parse();
    println!("Number: {}", cli.number);
}
