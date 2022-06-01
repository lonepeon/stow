package internal_test

import (
	"errors"
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
	"github.com/lonepeon/stow/internal/internaltest"
)

func TestBundleName(t *testing.T) {
	bundle := internal.NewBundle("vim")
	testutils.AssertEqualString(t, "vim", bundle.Name(), "invalid bundle name")
}

func TestBundleFiles(t *testing.T) {
	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	bundle := internal.NewBundle("vim")
	bundle.AddFile(vimrc)
	bundle.AddFile(goplugin)

	files := bundle.Files()
	testutils.RequireEqualInt(t, 2, len(files), "invalid file count")
	internaltest.AssertEqualPath(t, vimrc, files[0], "invalid vimrc file")
	internaltest.AssertEqualPath(t, goplugin, files[1], "invalid goplugin file")
}

func TestBuildBundleSuccess(t *testing.T) {
	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	walker := internaltest.NewWalker(vimrc, goplugin)
	bundle, err := internal.BuildBundleWithWalker(internal.Path("/home/user"), "vim", walker.Walk)
	testutils.RequireNoError(t, err, "unexpected error")

	testutils.AssertEqualString(t, "vim", bundle.Name(), "invalid bundle name")

	files := bundle.Files()
	testutils.RequireEqualInt(t, 2, len(files), "invalid file count")
	internaltest.AssertEqualPath(t, vimrc, files[0], "invalid vimrc file")
	internaltest.AssertEqualPath(t, goplugin, files[1], "invalid goplugin file")
}

func TestBuildBundleError(t *testing.T) {
	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	walker := internaltest.NewWalker(vimrc, goplugin)
	walker.SetErrorAtIndex(1, errors.New("boom"))

	_, err := internal.BuildBundleWithWalker(internal.Path("/home/user"), "vim", walker.Walk)
	testutils.RequireHasError(t, err, "expecting error")
	testutils.AssertContainsString(t, "boom", err.Error(), "invalid error")
}
