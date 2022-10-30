use clap::Parser;

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

fn main() {
    let cli = Cli::parse();

    println!(
        "symlink {} packages from {} onto {}",
        cli.packages.join(", "),
        cli.source_directory,
        cli.destination_directory
    );
}
