package internal

import (
	"fmt"
	"os"
)

type FileSystemLinker struct{}

func NewFileSystemLinker() *FileSystemLinker {
	return &FileSystemLinker{}
}

func (l FileSystemLinker) Link(src Path, dest Path) error {
	_, err := os.Stat(src.String())
	if err != nil {
		return fmt.Errorf("can't get src information: %v", err)
	}

	err = l.mkdirParents(dest)
	if err != nil {
		return fmt.Errorf("can't create dest parent folder: %v", err)
	}

	return l.overwriteSymlink(src, dest)
}

func (FileSystemLinker) Unlink(path Path) error {
	err := os.Remove(path.String())
	if err != nil {
		return fmt.Errorf("can't remove path link: %v", err)
	}

	return nil
}

func (l FileSystemLinker) mkdirParents(path Path) error {
	err := os.MkdirAll(path.Dir().String(), 0755)
	if err != nil {
		return fmt.Errorf("can't create parent folders: %v", err)
	}

	return nil
}

func (l FileSystemLinker) fileExists(path Path) (bool, error) {
	_, err := os.Stat(path.String())
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
		err = os.Remove(dest.String())
		if err != nil {
			return fmt.Errorf("can't remove existing dest link: %v", err)
		}
	}

	return os.Symlink(src.String(), dest.String())
}

func (l FileSystemLinker) ReadLink(path Path) (Path, error) {
	dest, err := os.Readlink(path.String())
	if err != nil {
		return Path(""), fmt.Errorf("can't read link: %w: %v", ErrLinkNotExist, err)
	}

	return Path(dest), nil
}
