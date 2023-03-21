[![codecov](https://codecov.io/gh/lonepeon/stow/branch/main/graph/badge.svg?token=HC7WSVDRO2)](https://codecov.io/gh/lonepeon/stow)

# Stow

Inspired from GNU stow, the goal of this utility is to be able to symlink
packages in target folders.

I used to have a `dotup` script which would copy my whole environment but this
solution is not flexible enough now I have to work with different computers:
personal Mac, company's Mac, company's development VM on Linux.

## Usage

```
stow $HOME vim
stow $HOME zsh/zshrc.mac
stow -D $HOME emacs
```

### Documentation

Documentation is available using `stow --help`

```
The command line is in charge of symlinking files from the STOW directory to a
target directory

It helps manage packages individually while still being able to install them in a
shared file tree.

Usage: stow [OPTIONS] -t <TARGET_DIRECTORY> [PACKAGES]...

Arguments:
  [PACKAGES]...
          Packages are all the directories placed at the root of the STOW_DIR.
          The content of these packages are all files and directories below these top level
          directories. They will be copied verbatim to the target directory.

Options:
  -D
          Tries to remove all the symlinks belonging to the targeted packages.

          For each package, gather all the directories containing configuration. In the
          targeted directory, remove all symlinks stored in directories named after the
          directories collected in the previous step, if they target the package.

  -d <SOURCE_DIRECTORY>
          This is the directory where packages can be found.
          Set the stow directory instead of using the STOW_DIR environment variable or the
          current directory.

          [env: STOW_DIR=~/Workspaces/lonepeon/dotfiles]
          [default: .]

  -t <TARGET_DIRECTORY>
          Target is the root directory where the content of the packages will be symlinked
          to. If a directory required by a package doesn't exist, it will be created
          automatically.

          [env: HOME=~]

  -n
          Do not execute the program, only print commands.

  -v <VERBOSITY>
          0: do not print anything to STDERR
          1: print only when the program will override a file or a symlink
          2: print all commands the program will execute to STDERR

          [default: 1]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

## Development

- Setup your local environment using `make setup`
- Run `make watch` to continuously run Clippy and the unit tests
