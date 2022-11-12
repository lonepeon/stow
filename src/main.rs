use clap::Parser;
use stow::command;
use stow::linker;
use stow::path;
use stow::writer;

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
        write!(f, "verbosity is out of range\n{}", VERSBOSITY_LONG_HELP)
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
#[command(author, version, about, long_about=ABOUT_LONG_HELP)]
struct Cli {
    #[arg(
        short = 'D',
        long = None,
        default_value="false",
        help = DELETE_SHORT_HELP,
        long_help = DELETE_LONG_HELP,
    )]
    delete: bool,
    #[arg(
        short = 'd',
        long = None,
        default_value=".",
        env = "STOW_DIR",
        help = SOURCE_SHORT_HELP,
        long_help = SOURCE_LONG_HELP,
    )]
    source_directory: String,
    #[arg(
        short = 't',
        long = None,
        env = "HOME",
        help = TARGET_SHORT_HELP,
        long_help=TARGET_LONG_HELP,
    )]
    target_directory: String,
    #[arg(short = 'n', help = DRY_RUN_SHORT_HELP, long_help=DRY_RUN_LONG_HELP)]
    dry_run: bool,
    #[arg(
        short = 'v',
        default_value = "1",
        value_parser=parse_verbosity,
        help = VERBOSITY_SHORT_HELP,
        long_help = VERSBOSITY_LONG_HELP
    )]
    verbosity: Verbosity,
    #[arg(help = PACKAGES_SHORT_HELP, long_help=PACKAGES_LONG_HELP)]
    packages: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let source_directory: path::Source = cli.source_directory.as_str().into();
    let destination_directory: path::Destination = cli.target_directory.as_str().into();

    let stderr = std::io::stderr();
    let mut link: Box<dyn linker::Linker> = if cli.dry_run {
        Box::new(linker::Verbose::new(&stderr, linker::Noop::default()))
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

    let mut cmd = command::Command::new(command_logger, link.as_mut());
    if cli.delete {
        cmd.unstow(&source_directory, &destination_directory, cli.packages)?
    } else {
        cmd.stow(&source_directory, &destination_directory, cli.packages)?;
    }

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

static ABOUT_LONG_HELP: &str =
    "The command line is in charge of symlinking files from the STOW directory to a
target directory

It helps manage packages individually while still being able to install them in a
shared file tree.";

static DELETE_SHORT_HELP: &str = "Delete all the symlinks pointing to the targeted packages";

static DELETE_LONG_HELP: &str =
    "Tries to remove all the symlinks belonging to the targeted packages.

For each package, gather all the directories containing configuration. In the
targeted directory, remove all symlinks stored in directories named after the
directories collected in the previous step, if they target the package.";

static DRY_RUN_SHORT_HELP: &str = "Do not execute the program, only print commands";

static DRY_RUN_LONG_HELP: &str = "Do not execute the program, only print commands.";

static PACKAGES_SHORT_HELP: &str = "All the packages to install on or remove from the system";

static PACKAGES_LONG_HELP: &str =
    "Packages are all the directories placed at the root of the STOW_DIR.
The content of these packages are all files and directories below these top level
directories. They will be copied verbatim to the target directory.";

static SOURCE_SHORT_HELP: &str = "Set the directory where packages can be found";

static SOURCE_LONG_HELP: &str = "This is the directory where packages can be found.
Set the stow directory instead of using the STOW_DIR environment variable or the
current directory.";

static TARGET_SHORT_HELP: &str = "Set the directory where files will be placed";

static TARGET_LONG_HELP: &str =
    "Target is the root directory where the content of the packages will be symlinked
to. If a directory required by a package doesn't exist, it will be created
automatically.";

static VERBOSITY_SHORT_HELP: &str = "From quiet(0) to chatty(2)";

static VERSBOSITY_LONG_HELP: &str = "0: do not print anything to STDERR
1: print only when the program will override a file or a symlink
2: print all commands the program will execute to STDERR";
