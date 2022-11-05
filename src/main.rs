use clap::Parser;
use stow::linker;
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
    #[arg(short = 'n', help = "Do not execute the program, only print commands")]
    dry_run: bool,
    #[arg(help = "All the packages to install on or remove from the system")]
    packages: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let source_directory: path::Source = cli.source_directory.as_str().into();
    let destination_directory: path::Destination = cli.destination_directory.as_str().into();

    let stderr = std::io::stderr();
    let mut link: Box<dyn linker::Linker> = if cli.dry_run {
        Box::new(linker::DryRunLinker::new(&stderr))
    } else {
        Box::new(linker::OSLinker::default())
    };

    linker::copy(
        link.as_mut(),
        &source_directory,
        &destination_directory,
        cli.packages,
    )?;

    Ok(())
}
