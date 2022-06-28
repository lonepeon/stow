package internal

import (
	"fmt"
	"io"
	"os"
)

type FileSystemExecutor interface {
	MkdirAll(path Path, mode os.FileMode) error
	Stat(Path) (os.FileInfo, error)
	Readlink(Path) (Path, error)
	Remove(Path) error
	Symlink(src, dest Path) error
}

type LocalFileSystemExecutor struct{}

func NewLocalFileSystemExecutor() LocalFileSystemExecutor {
	return LocalFileSystemExecutor{}
}

func (LocalFileSystemExecutor) Stat(path Path) (os.FileInfo, error) {
	return os.Stat(path.String())
}

func (LocalFileSystemExecutor) Remove(path Path) error {
	return os.Remove(path.String())
}

func (LocalFileSystemExecutor) MkdirAll(path Path, mode os.FileMode) error {
	return os.MkdirAll(path.String(), mode)
}

func (LocalFileSystemExecutor) Readlink(path Path) (Path, error) {
	dest, err := os.Readlink(path.String())
	return Path(dest), err
}

func (LocalFileSystemExecutor) Symlink(src, dest Path) error {
	return os.Symlink(src.String(), dest.String())
}

type VerboseFileSystemExecutor struct {
	logger   *Logger
	executor FileSystemExecutor
}

func NewVerboseFileSystemExecutor(logger *Logger, executor FileSystemExecutor) VerboseFileSystemExecutor {
	return VerboseFileSystemExecutor{logger: logger, executor: executor}
}

func (e VerboseFileSystemExecutor) Stat(path Path) (os.FileInfo, error) {
	e.logger.Infof("stat file %s", path)
	info, err := e.executor.Stat(path)
	if err != nil {
		e.logger.Debugf("failed to stat file: %v", err)
	}

	return info, err
}

func (e VerboseFileSystemExecutor) Remove(path Path) error {
	e.logger.Infof("rm file %s", path)
	err := e.executor.Remove(path)
	if err != nil {
		e.logger.Debugf("failed to remove file %s: %v", path, err)
	}

	return err
}

func (e VerboseFileSystemExecutor) MkdirAll(path Path, mode os.FileMode) error {
	e.logger.Infof("mkdir all %s with mode %v", path, mode)
	err := e.executor.MkdirAll(path, mode)
	if err != nil {
		e.logger.Debugf("failed to mkdir all %s: %v", path, err)
	}

	return err
}

func (e VerboseFileSystemExecutor) Readlink(path Path) (Path, error) {
	e.logger.Infof("read symlink info for %s", path)
	path, err := e.executor.Readlink(path)
	if err != nil {
		e.logger.Debugf("failed to read symlink info for %s: %v", path, err)
	}
	return path, err
}

func (e VerboseFileSystemExecutor) Symlink(src, dest Path) error {
	e.logger.Infof("symlink from %s to %s", src, dest)
	err := e.executor.Symlink(src, dest)
	if err != nil {
		e.logger.Debugf("failed to symlink from %s to %s: %v", src, dest, err)
	}

	return err
}

type DryRunFileSystemExecutor struct {
	out io.Writer
}

func NewDryRunFileSystemExecutor(out io.Writer) DryRunFileSystemExecutor {
	return DryRunFileSystemExecutor{out: out}
}

func (DryRunFileSystemExecutor) Stat(path Path) (os.FileInfo, error) {
	return os.Stat(path.String())
}

func (e DryRunFileSystemExecutor) Remove(path Path) error {
	fmt.Fprintf(e.out, "rm %s\n", path)
	return nil
}

func (e DryRunFileSystemExecutor) MkdirAll(path Path, mode os.FileMode) error {
	fmt.Fprintf(e.out, "mkdir -p %s (mode %v)\n", path, mode)
	return nil
}

func (DryRunFileSystemExecutor) Readlink(path Path) (Path, error) {
	dest, err := os.Readlink(path.String())
	return Path(dest), err
}

func (e DryRunFileSystemExecutor) Symlink(src, dest Path) error {
	fmt.Fprintf(e.out, "ln -s %s %s\n", src, dest)
	return nil
}
