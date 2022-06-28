package internal

import (
	"errors"
	"fmt"
	"strings"
)

var (
	ErrLinkNotExist = errors.New("link does not exist")
)

type Linker interface {
	// Link creates a symbolic link from src to dest. It also creates the
	// subdolders if necessary
	Link(src Path, dest Path) error
	// Unlink removes a symbolic link. Unlink(Path) error
	Unlink(Path) error
	// ReadLink reads the symlink and returns the path to the file it points
	// to. If link doesn't exist, returns an ErrLinkNotExist
	ReadLink(Path) (Path, error)
}

type RollbackError struct {
	errors []error
}

func (e *RollbackError) IsZero() bool {
	return len(e.errors) == 0
}

func (e *RollbackError) Append(err error) {
	e.errors = append(e.errors, err)
}

func (e *RollbackError) Error() string {
	var str strings.Builder
	for _, err := range e.errors {
		str.WriteString(err.Error())
	}

	return strings.TrimSpace(str.String())
}

type TargetLinkTransactionStatement struct {
	source      Path
	destination Path
}

type Target struct {
	root   Path
	linker Linker
}

func NewTargetWithLinker(root Path, linker Linker) *Target {
	return &Target{root: root, linker: linker}
}

func (t *Target) Stow(stowRoot Path, pkg *Package) error {
	var transaction []TargetLinkTransactionStatement
	for _, srcPath := range pkg.Files() {
		src := stowRoot.Join(Path(pkg.Name())).Join(srcPath)
		dest := t.root.Join(srcPath)

		err := t.linker.Link(src, dest)
		if err != nil {
			errRollback := t.rollbackLinking(transaction)
			if errRollback != nil {
				return fmt.Errorf("can't link '%s' -> %s: %v. rollback did not succeed: %v", src, dest, err, errRollback)
			}
			return fmt.Errorf("can't link '%s' -> %s: %v. rollback succeeded", src, dest, err)
		}

		transaction = append(transaction, TargetLinkTransactionStatement{
			source:      src,
			destination: dest,
		})
	}

	return nil
}

func (t *Target) Unstow(stowRoot Path, pkg *Package) error {
	var transaction []TargetLinkTransactionStatement
	for _, srcPath := range pkg.Files() {
		expectedSrc := stowRoot.Join(srcPath)
		dest := t.root.Join(srcPath)

		actualSrc, err := t.linker.ReadLink(dest)
		if err != nil {
			if errors.Is(err, ErrLinkNotExist) {
				continue
			}

			errRollback := t.rollbackUnlinking(transaction)
			if errRollback != nil {
				return fmt.Errorf("can't get link '%s' information: %v. rollback did not succeed: %v", dest, err, errRollback)
			}
			return fmt.Errorf("can't get link information '%s': %v. rollback succeeded", dest, err)
		}

		if actualSrc != expectedSrc {
			continue
		}

		err = t.linker.Unlink(dest)
		if err != nil {
			errRollback := t.rollbackUnlinking(transaction)
			if errRollback != nil {
				return fmt.Errorf("can't unlink %s: %v. rollback did not succeed: %v", dest, err, errRollback)
			}
			return fmt.Errorf("can't unlink '%s': %v. rollback succeeded", dest, err)
		}

		transaction = append(transaction, TargetLinkTransactionStatement{
			source:      expectedSrc,
			destination: dest,
		})
	}

	return nil
}

func (t *Target) rollbackLinking(transaction []TargetLinkTransactionStatement) error {
	var errs RollbackError
	for _, statement := range transaction {
		err := t.linker.Unlink(statement.destination)
		if err != nil {
			errs.Append(fmt.Errorf("can't unlink '%s': %v", statement.destination, err))
			continue
		}
	}

	if !errs.IsZero() {
		return &errs
	}

	return nil
}

func (t *Target) rollbackUnlinking(transaction []TargetLinkTransactionStatement) error {
	var errs RollbackError
	for _, statement := range transaction {
		err := t.linker.Link(statement.source, statement.destination)
		if err != nil {
			errs.Append(fmt.Errorf("can't re-link '%s' -> '%s': %v", statement.source, statement.destination, err))
			continue
		}
	}

	if !errs.IsZero() {
		return &errs
	}

	return nil
}
