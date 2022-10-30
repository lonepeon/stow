use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {}

fn main() {
    Cli::parse();

    println!("Hello, world!");
}
