package internaltest

import "github.com/lonepeon/stow/internal"

type Link struct {
	Source      internal.Path
	Destination internal.Path
}

type Linker struct {
	linkErrAtIndex int
	linkErr        error
	links          []Link

	unlinkErrAtIndex int
	unlinkErr        error
	unlinks          []internal.Path
}

func NewLinker() *Linker {
	return &Linker{
		linkErrAtIndex: -1,
		linkErr:        nil,
		links:          nil,

		unlinkErrAtIndex: -1,
		unlinkErr:        nil,
		unlinks:          nil,
	}
}

func (l *Linker) SetLinkErrorAtIndex(i int, err error) {
	l.linkErrAtIndex = i
	l.linkErr = err
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
