# Stow

Inspired from GNU stow, the goal of this utility is to be able to symlink packages in target folders. I used to have a `dotup` script which would copy my whole environment but this solution is not flexible enough now I have to work with different computers: personal Mac, company's Mac, company's development VM on Linux.

## Usage

```
stow $HOME vim
stow $HOME zsh/zshrc.mac
stow -D $HOME emacs
```
