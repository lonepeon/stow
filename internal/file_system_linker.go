package internal

import (
	"fmt"
	"os"
)

type FileSystemLinker struct {
	executor FileSystemExecutor
}

func NewFileSystemLinker(executor FileSystemExecutor) *FileSystemLinker {
	return &FileSystemLinker{executor: executor}
}

func (l FileSystemLinker) Link(src Path, dest Path) error {
	_, err := l.executor.Stat(src)
	if err != nil {
		return fmt.Errorf("can't get src information: %v", err)
	}

	err = l.mkdirParents(dest)
	if err != nil {
		return fmt.Errorf("can't create dest parent folder: %v", err)
	}

	return l.overwriteSymlink(src, dest)
}

func (l FileSystemLinker) Unlink(path Path) error {
	err := l.executor.Remove(path)
	if err != nil {
		return fmt.Errorf("can't remove path link: %v", err)
	}

	return nil
}

func (l FileSystemLinker) mkdirParents(path Path) error {
	err := l.executor.MkdirAll(path.Dir(), 0755)
	if err != nil {
		return fmt.Errorf("can't create parent folders: %v", err)
	}

	return nil
}

func (l FileSystemLinker) fileExists(path Path) (bool, error) {
	_, err := l.executor.Stat(path)
	if err != nil {
		if !os.IsNotExist(err) {
			return false, nil
		}

		return false, err
	}

	return true, nil
}

func (l FileSystemLinker) overwriteSymlink(src Path, dest Path) error {
	destExists, err := l.fileExists(dest)
	if err != nil && !os.IsNotExist(err) {
		return fmt.Errorf("can't get dest information: %v", err)
	}

	if destExists {
		err = l.executor.Remove(dest)
		if err != nil {
			return fmt.Errorf("can't remove existing dest link: %v", err)
		}
	}

	return l.executor.Symlink(src, dest)
}

func (l FileSystemLinker) ReadLink(path Path) (Path, error) {
	dest, err := l.executor.Readlink(path)
	if err != nil {
		return Path(""), fmt.Errorf("can't read link: %w: %v", ErrLinkNotExist, err)
	}

	return dest, nil
}
