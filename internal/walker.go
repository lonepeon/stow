package internal

import (
	"io/fs"
	"path/filepath"
)

type WalkerFunc func(Path)

func WalkFileSystem(root Path, fn WalkerFunc) error {
	return filepath.WalkDir(root.String(), func(fpath string, fdir fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if fdir.IsDir() {
			return nil
		}

		path, err := filepath.Rel(root.String(), fpath)
		if err != nil {
			return err
		}

		fn(Path(path))

		return nil
	})
}
