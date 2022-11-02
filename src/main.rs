use clap::Parser;
use stow::path;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(
        short = 'd',
        long = None,
        default_value=".",
        env = "STOW_DIR",
        help = "Set the stow directory instead of using the STOW_DIR environment variable or the current directory"
    )]
    source_directory: String,
    #[arg(
        short = 't',
        long = None,
        env = "HOME",
        help = "Set the target directory instead of using the HOME folder"
    )]
    destination_directory: String,
    #[arg(help = "All the packages to install on or remove from the system")]
    packages: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let source_directory = cli.source_directory.parse::<path::Path>()?;
    let destination_directory = cli.destination_directory.parse::<path::Path>()?;

    println!(
        "symlink {} packages from {} onto {}",
        cli.packages.join(", "),
        source_directory,
        destination_directory
    );

    Ok(())
}
