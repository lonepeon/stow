package internaltest

import (
	"github.com/lonepeon/stow/internal"
)

type Walker struct {
	paths      []internal.Path
	err        error
	errAtIndex int
}

func NewWalker(paths ...internal.Path) *Walker {
	return &Walker{
		paths:      paths,
		err:        nil,
		errAtIndex: -1,
	}
}

func (w *Walker) SetErrorAtIndex(index int, err error) {
	w.err = err
	w.errAtIndex = index
}

func (w *Walker) Walk(_ internal.Path, fn internal.WalkerFunc) error {
	for i, path := range w.paths {
		if i == w.errAtIndex {
			return w.err
		}

		fn(path)
	}

	return nil
}
