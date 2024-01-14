use clap::Parser;
use rustyc_driver::Driver;

#[derive(Parser)]
#[command(author = "ydolev", version = "0.1.0", about = "A minimalist C compiler written in Rust", long_about = None)]
struct Cli {
    expression: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut driver = Driver::new(cli.expression);
    driver.run()?;

    Ok(())
}
