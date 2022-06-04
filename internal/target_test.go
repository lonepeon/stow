package internal_test

import (
	"errors"
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
	"github.com/lonepeon/stow/internal/internaltest"
)

func TestTargetStowSuccess(t *testing.T) {
	stowPath := internal.Path("/usr/local/var/stow")
	targetPath := internal.Path("/home/user")

	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	linker := internaltest.NewLinker()
	pkg := internal.NewPackage("vim")
	pkg.AddFile(vimrc)
	pkg.AddFile(goplugin)

	target := internal.NewTargetWithLinker(targetPath, linker)
	err := target.Stow(stowPath, pkg)
	testutils.RequireNoError(t, err, "unexpected error")

	testutils.RequireEqualInt(t, 2, linker.LenLinks(), "invalid number of linked files")
	testutils.RequireEqualInt(t, 0, linker.LenUnlinks(), "invalid number of unlinked files")
	internaltest.AssertEqualPath(t, stowPath.Join(vimrc), linker.GetLink(0).Source, "invalid vimrc source link")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetLink(0).Destination, "invalid vimrc destination link")
	internaltest.AssertEqualPath(t, stowPath.Join(goplugin), linker.GetLink(1).Source, "invalid goplugin source link")
	internaltest.AssertEqualPath(t, targetPath.Join(goplugin), linker.GetLink(1).Destination, "invalid goplugin destination link")
}

func TestTargetStowFailButRollback(t *testing.T) {
	stowPath := internal.Path("/usr/local/var/stow")
	targetPath := internal.Path("/home/user")

	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	linker := internaltest.NewLinker()
	linker.SetLinkErrorAtIndex(1, errors.New("boom"))

	pkg := internal.NewPackage("vim")
	pkg.AddFile(vimrc)
	pkg.AddFile(goplugin)

	target := internal.NewTargetWithLinker(targetPath, linker)
	err := target.Stow(stowPath, pkg)
	testutils.RequireHasError(t, err, "expecting an error")
	testutils.AssertContainsString(t, "boom", err.Error(), "invalid string error")
	testutils.AssertContainsString(t, stowPath.Join(goplugin).String(), err.Error(), "invalid string error")
	testutils.AssertContainsString(t, targetPath.Join(goplugin).String(), err.Error(), "invalid string error")
	testutils.AssertContainsString(t, "rollback succeeded", err.Error(), "invalid string error")

	testutils.RequireEqualInt(t, 1, linker.LenLinks(), "invalid number of linked files")
	testutils.RequireEqualInt(t, 1, linker.LenUnlinks(), "invalid number of unlinked files")
	internaltest.AssertEqualPath(t, stowPath.Join(vimrc), linker.GetLink(0).Source, "invalid vimrc source link")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetLink(0).Destination, "invalid vimrc destination link")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetUnlink(0), "invalid vimrc unlink")
}

func TestTargetStowFailAndRollbackFail(t *testing.T) {
	stowPath := internal.Path("/usr/local/var/stow")
	targetPath := internal.Path("/home/user")

	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")
	godetect := internal.Path(".vim/after/ftdetect/go.vim")

	linker := internaltest.NewLinker()
	linker.SetLinkErrorAtIndex(2, errors.New("boom"))
	linker.SetUnlinkErrorAtIndex(1, errors.New("bam"))

	pkg := internal.NewPackage("vim")
	pkg.AddFile(vimrc)
	pkg.AddFile(goplugin)
	pkg.AddFile(godetect)

	target := internal.NewTargetWithLinker(targetPath, linker)
	err := target.Stow(stowPath, pkg)
	testutils.RequireHasError(t, err, "expecting an error")
	testutils.AssertContainsString(t, "boom", err.Error(), "invalid string error")
	testutils.AssertContainsString(t, stowPath.Join(godetect).String(), err.Error(), "invalid string error")
	testutils.AssertContainsString(t, targetPath.Join(godetect).String(), err.Error(), "invalid string error")
	testutils.AssertContainsString(t, "bam", err.Error(), "invalid string error")
	testutils.AssertContainsString(t, "rollback did not succeed", err.Error(), "invalid string error")

	testutils.RequireEqualInt(t, 2, linker.LenLinks(), "invalid number of linked files")
	testutils.RequireEqualInt(t, 1, linker.LenUnlinks(), "invalid number of unlinked files")
	internaltest.AssertEqualPath(t, stowPath.Join(vimrc), linker.GetLink(0).Source, "invalid vimrc source link")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetLink(0).Destination, "invalid vimrc destination link")
	internaltest.AssertEqualPath(t, stowPath.Join(goplugin), linker.GetLink(1).Source, "invalid vimrc source link")
	internaltest.AssertEqualPath(t, targetPath.Join(goplugin), linker.GetLink(1).Destination, "invalid goplugin destination link")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetUnlink(0), "invalid vimrc unlink")
}

func TestTargetUnstowSuccess(t *testing.T) {
	stowPath := internal.Path("/usr/local/var/stow")
	targetPath := internal.Path("/home/user")

	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	linker := internaltest.NewLinker()
	linker.RegisterReadLink(stowPath.Join(vimrc), targetPath.Join(vimrc))
	linker.RegisterReadLink(stowPath.Join(goplugin), targetPath.Join(goplugin))

	pkg := internal.NewPackage("vim")
	pkg.AddFile(vimrc)
	pkg.AddFile(goplugin)

	target := internal.NewTargetWithLinker(targetPath, linker)
	err := target.Unstow(stowPath, pkg)
	testutils.RequireNoError(t, err, "unexpected error")

	testutils.RequireEqualInt(t, 2, linker.LenUnlinks(), "invalid number of unlinked files")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetUnlink(0), "invalid vimrc unlink")
	internaltest.AssertEqualPath(t, targetPath.Join(goplugin), linker.GetUnlink(1), "invalid golugin unlink")
}

func TestTargetUnstowSuccessWhenFileMissing(t *testing.T) {
	stowPath := internal.Path("/usr/local/var/stow")
	targetPath := internal.Path("/home/user")

	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	linker := internaltest.NewLinker()
	linker.RegisterReadLink(stowPath.Join(vimrc), targetPath.Join(vimrc))

	pkg := internal.NewPackage("vim")
	pkg.AddFile(vimrc)
	pkg.AddFile(goplugin)

	target := internal.NewTargetWithLinker(targetPath, linker)
	err := target.Unstow(stowPath, pkg)
	testutils.RequireNoError(t, err, "unexpected error")

	testutils.RequireEqualInt(t, 1, linker.LenUnlinks(), "invalid number of unlinked files")
	testutils.RequireEqualInt(t, 0, linker.LenLinks(), "invalid number of linked files")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetUnlink(0), "invalid vimrc unlink")
}

func TestTargetUnstowFailButRollbackSuccess(t *testing.T) {
	stowPath := internal.Path("/usr/local/var/stow")
	targetPath := internal.Path("/home/user")

	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	linker := internaltest.NewLinker()
	linker.RegisterReadLink(stowPath.Join(vimrc), targetPath.Join(vimrc))
	linker.RegisterReadLink(stowPath.Join(goplugin), targetPath.Join(goplugin))
	linker.SetUnlinkErrorAtIndex(1, errors.New("boom"))

	pkg := internal.NewPackage("vim")
	pkg.AddFile(vimrc)
	pkg.AddFile(goplugin)

	target := internal.NewTargetWithLinker(targetPath, linker)
	err := target.Unstow(stowPath, pkg)
	testutils.RequireHasError(t, err, "expecting an error")

	testutils.AssertContainsString(t, "unlink", err.Error(), "invalid error message")
	testutils.AssertContainsString(t, "rollback succeeded", err.Error(), "invalid error message")
	testutils.AssertContainsString(t, "boom", err.Error(), "invalid error message")
	testutils.AssertContainsString(t, targetPath.Join(goplugin).String(), err.Error(), "invalid error message")

	testutils.RequireEqualInt(t, 1, linker.LenUnlinks(), "invalid number of unlinked files")
	testutils.RequireEqualInt(t, 1, linker.LenLinks(), "invalid number of linked files")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetUnlink(0), "invalid vimrc unlink")
	internaltest.AssertEqualPath(t, stowPath.Join(vimrc), linker.GetLink(0).Source, "invalid vimrc link source")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetLink(0).Destination, "invalid vimrc link destination")
}

func TestTargetUnstowFailButRollbackFailed(t *testing.T) {
	stowPath := internal.Path("/usr/local/var/stow")
	targetPath := internal.Path("/home/user")

	vimrc := internal.Path(".vim/vimrc")
	goplugin := internal.Path(".vim/after/ftplugin/go.vim")

	linker := internaltest.NewLinker()
	linker.RegisterReadLink(stowPath.Join(vimrc), targetPath.Join(vimrc))
	linker.RegisterReadLink(stowPath.Join(goplugin), targetPath.Join(goplugin))
	linker.SetUnlinkErrorAtIndex(1, errors.New("boom"))
	linker.SetLinkErrorAtIndex(0, errors.New("bam"))

	pkg := internal.NewPackage("vim")
	pkg.AddFile(vimrc)
	pkg.AddFile(goplugin)

	target := internal.NewTargetWithLinker(targetPath, linker)
	err := target.Unstow(stowPath, pkg)
	testutils.RequireHasError(t, err, "expecting an error")

	testutils.AssertContainsString(t, "unlink", err.Error(), "invalid error message")
	testutils.AssertContainsString(t, "rollback did not succeed", err.Error(), "invalid error message")
	testutils.AssertContainsString(t, "boom", err.Error(), "invalid error message")
	testutils.AssertContainsString(t, "bam", err.Error(), "invalid error message")
	testutils.AssertContainsString(t, targetPath.Join(goplugin).String(), err.Error(), "invalid error message")
	testutils.AssertContainsString(t, stowPath.Join(vimrc).String(), err.Error(), "invalid error message")
	testutils.AssertContainsString(t, targetPath.Join(vimrc).String(), err.Error(), "invalid error message")

	testutils.RequireEqualInt(t, 1, linker.LenUnlinks(), "invalid number of unlinked files")
	testutils.RequireEqualInt(t, 0, linker.LenLinks(), "invalid number of linked files")
	internaltest.AssertEqualPath(t, targetPath.Join(vimrc), linker.GetUnlink(0), "invalid vimrc unlink")
}
