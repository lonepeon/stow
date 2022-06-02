package internal_test

import (
	"errors"
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
	"github.com/lonepeon/stow/internal/internaltest"
)

func TestPackageName(t *testing.T) {
	pkg := internal.NewPackage("vim")
	testutils.AssertEqualString(t, "vim", pkg.Name(), "invalid package name")
}

func TestPackageFiles(t *testing.T) {
	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	pkg := internal.NewPackage("vim")
	pkg.AddFile(vimrc)
	pkg.AddFile(goplugin)

	files := pkg.Files()
	testutils.RequireEqualInt(t, 2, len(files), "invalid file count")
	internaltest.AssertEqualPath(t, vimrc, files[0], "invalid vimrc file")
	internaltest.AssertEqualPath(t, goplugin, files[1], "invalid goplugin file")
}

func TestBuildPackageSuccess(t *testing.T) {
	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	walker := internaltest.NewWalker(vimrc, goplugin)
	pkg, err := internal.BuildPackageWithWalker(internal.Path("/home/user"), "vim", walker.Walk)
	testutils.RequireNoError(t, err, "unexpected error")

	testutils.AssertEqualString(t, "vim", pkg.Name(), "invalid package name")

	files := pkg.Files()
	testutils.RequireEqualInt(t, 2, len(files), "invalid file count")
	internaltest.AssertEqualPath(t, vimrc, files[0], "invalid vimrc file")
	internaltest.AssertEqualPath(t, goplugin, files[1], "invalid goplugin file")
}

func TestBuildPackageError(t *testing.T) {
	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	walker := internaltest.NewWalker(vimrc, goplugin)
	walker.SetErrorAtIndex(1, errors.New("boom"))

	_, err := internal.BuildPackageWithWalker(internal.Path("/home/user"), "vim", walker.Walk)
	testutils.RequireHasError(t, err, "expecting error")
	testutils.AssertContainsString(t, "boom", err.Error(), "invalid error")
}
