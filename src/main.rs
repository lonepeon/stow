use clap::Parser;
use stow::command;
use stow::linker;
use stow::path;
use stow::writer;

static VERSBOSITY_HELP: &str = "0: do not print anything to STDERR
1: print only when the command will override a file/symlink with a different value
2: print every command to STDERR";

#[derive(Debug, PartialEq, Clone)]
enum Verbosity {
    Silent,
    WarningOnly,
    Verbose,
}

#[derive(Debug, PartialEq)]
struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "verbosity is out of range\n{}", VERSBOSITY_HELP)
    }
}

impl std::error::Error for Error {}

fn parse_verbosity(arg: &str) -> Result<Verbosity, Error> {
    let verbosity = arg.parse::<u8>().map_err(|_| Error)?;
    match verbosity {
        0 => Ok(Verbosity::Silent),
        1 => Ok(Verbosity::WarningOnly),
        2 => Ok(Verbosity::Verbose),
        _ => Err(Error),
    }
}

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
    #[arg(
        short = 'v',
        default_value = "1",
        value_parser=parse_verbosity,
        help = "From quiet (0) to chatty (2)",
        long_help = VERSBOSITY_HELP
    )]
    verbosity: Verbosity,
    #[arg(help = "All the packages to install on or remove from the system")]
    packages: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let source_directory: path::Source = cli.source_directory.as_str().into();
    let destination_directory: path::Destination = cli.destination_directory.as_str().into();

    let stderr = std::io::stderr();
    let mut link: Box<dyn linker::Linker> = if cli.dry_run {
        Box::new(linker::Verbose::new(&stderr, linker::Noop))
    } else if cli.verbosity == Verbosity::Verbose {
        Box::new(linker::Verbose::new(&stderr, linker::Filesystem))
    } else {
        Box::new(linker::Filesystem)
    };

    let command_logger: Box<dyn std::io::Write> = if cli.verbosity == Verbosity::Silent {
        Box::new(writer::Noop)
    } else {
        Box::new(&stderr)
    };

    command::Command::new(command_logger, link.as_mut()).stow(
        &source_directory,
        &destination_directory,
        cli.packages,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_invalid_verbosity() {
        let testcases = vec![
            ("-1", "negative value"),
            ("quiet", "string value"),
            ("5", "out of range value"),
        ];

        for (value, reason) in testcases {
            assert_eq!(
                Err(Error),
                parse_verbosity(value),
                "the parser did not fail to parse {}",
                reason
            )
        }
    }

    #[test]
    fn parse_valid_verbosity() {
        let testcases = vec![
            ("0", Verbosity::Silent),
            ("1", Verbosity::WarningOnly),
            ("2", Verbosity::Verbose),
        ];

        for (value, expected) in testcases {
            assert_eq!(Ok(expected), parse_verbosity(value))
        }
    }
}
