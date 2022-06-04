package internaltest

import (
	"github.com/lonepeon/stow/internal"
)

type Link struct {
	Source      internal.Path
	Destination internal.Path
}

type Linker struct {
	linkErrAtIndex int
	linkErr        error
	links          []Link

	readlinkErrAtIndex int
	readlinkErr        error
	readlinks          []Link

	unlinkErrAtIndex int
	unlinkErr        error
	unlinks          []internal.Path
}

func NewLinker() *Linker {
	return &Linker{
		linkErrAtIndex: -1,
		linkErr:        nil,
		links:          nil,

		readlinkErrAtIndex: -1,
		readlinkErr:        nil,
		readlinks:          nil,

		unlinkErrAtIndex: -1,
		unlinkErr:        nil,
		unlinks:          nil,
	}
}

func (l *Linker) SetLinkErrorAtIndex(i int, err error) {
	l.linkErrAtIndex = i
	l.linkErr = err
}

func (l *Linker) SetReadlinkErrorAtIndex(i int, err error) {
	l.readlinkErrAtIndex = i
	l.readlinkErr = err
}

func (l *Linker) RegisterReadLink(src internal.Path, dest internal.Path) {
	l.readlinks = append(l.readlinks, Link{
		Source:      src,
		Destination: dest,
	})
}

func (l *Linker) SetUnlinkErrorAtIndex(i int, err error) {
	l.unlinkErrAtIndex = i
	l.unlinkErr = err
}

func (l *Linker) Link(src, dest internal.Path) error {
	if len(l.links) == l.linkErrAtIndex {
		return l.linkErr
	}

	l.links = append(l.links, Link{
		Source:      src,
		Destination: dest,
	})

	return nil
}

func (l *Linker) ReadLink(path internal.Path) (internal.Path, error) {
	for i, link := range l.readlinks {
		if i == l.readlinkErrAtIndex {
			return internal.Path(""), l.readlinkErr
		}

		if link.Destination != path {
			continue
		}

		return link.Source, nil
	}

	return internal.Path(""), internal.ErrLinkNotExist
}

func (l *Linker) Unlink(path internal.Path) error {
	if len(l.unlinks) == l.unlinkErrAtIndex {
		return l.unlinkErr
	}

	l.unlinks = append(l.unlinks, path)

	return nil
}

func (l *Linker) LenLinks() int {
	return len(l.links)
}

func (l *Linker) GetLink(i int) Link {
	return l.links[i]
}

func (l *Linker) LenUnlinks() int {
	return len(l.unlinks)
}

func (l *Linker) GetUnlink(i int) internal.Path {
	return l.unlinks[i]
}
