package internal_test

import (
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
	"github.com/lonepeon/stow/internal/internaltest"
)

func TestName(t *testing.T) {
	bundle := internal.NewBundle("vim")
	testutils.AssertEqualString(t, "vim", bundle.Name(), "invalid bundle name")
}

func TestFiles(t *testing.T) {
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
