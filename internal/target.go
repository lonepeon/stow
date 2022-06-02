package internal

import "fmt"

type Linker interface {
	// Link creates a symbolic link from src to dest.
	// It also creates the subdolders if necessary
	Link(src Path, dest Path) error
	// Unlink removes a symbolic link.
	Unlink(Path) error
}

type Target struct {
	root   Path
	linker Linker
}

func NewTarget(root Path) *Target {
	return NewTargetWithLinker(root, nil)
}

func NewTargetWithLinker(root Path, linker Linker) *Target {
	return &Target{root: root, linker: linker}
}

func (t *Target) Stow(srcRoot Path, pkg *Package) error {
	for i, srcPath := range pkg.Files() {
		src := srcRoot.Join(srcPath)
		dest := t.root.Join(srcPath)
		err := t.linker.Link(src, dest)
		if err != nil {
			errRollback := t.rollbackUpto(pkg, i)
			if errRollback != nil {
				return fmt.Errorf("can't link '%s' -> %s: %v. rollback did not succeed: %v", src, dest, err, errRollback)
			}
			return fmt.Errorf("can't link '%s' -> %s: %v. rollback succeeded", src, dest, err)
		}
	}

	return nil
}

func (t *Target) rollbackUpto(b *Package, index int) error {
	for i, src := range b.Files() {
		if i >= index {
			return nil
		}

		dest := t.root.Join(src)
		err := t.linker.Unlink(dest)
		if err != nil {
			return fmt.Errorf("can't rollbacl '%s': %v", dest, err)
		}
	}

	return nil
}
