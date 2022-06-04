package internal_test

import (
	"os"
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
	"github.com/lonepeon/stow/internal/internaltest"
)

func TestFileSystemLinkerLink(t *testing.T) {
	destFolder, err := os.MkdirTemp("", "testlink")
	testutils.RequireNoError(t, err, "can't create temp folder %v", destFolder)
	defer os.Remove(destFolder)

	destFolderPath := internal.Path(destFolder)

	tcs := map[string]struct {
		source      internal.Path
		destination internal.Path
	}{
		"no-subfolder": {
			source:      internal.Path("testdata/package1/file-1.txt"),
			destination: destFolderPath.Join(internal.Path("file-1.txt")),
		},
		"with-subfolder": {
			source:      internal.Path("testdata/package1/subfolder/file-3.txt"),
			destination: destFolderPath.Join(internal.Path("subfolder/file-3.txt")),
		},
	}

	for name, tc := range tcs {
		t.Run(name, func(t *testing.T) {
			fs := internal.NewFileSystemLinker()
			err := fs.Link(tc.source, tc.destination)
			testutils.RequireNoError(t, err, "unexpected linking error")

			srcPath, err := os.Readlink(tc.destination.String())
			testutils.RequireNoError(t, err, "unexpected destination stat error")

			testutils.AssertEqualString(t, tc.source.String(), srcPath, "invalid link destination")
		})
	}
}

func TestFileSystemLinkerUnlink(t *testing.T) {
	destFolder, err := os.MkdirTemp("", "testunlink")
	testutils.RequireNoError(t, err, "can't create temp folder %v", destFolder)
	defer os.Remove(destFolder)

	destFolderPath := internal.Path(destFolder)

	source := internal.Path("testdata/package1/file-1.txt")
	destination := destFolderPath.Join(internal.Path("file-1.txt"))

	fs := internal.NewFileSystemLinker()
	err = fs.Link(source, destination)
	testutils.RequireNoError(t, err, "unexpected linking error")

	_, err = os.Lstat(destination.String())
	testutils.RequireNoError(t, err, "unexpected error on destination stat")

	err = fs.Unlink(destination)
	testutils.RequireNoError(t, err, "unexpected unlinking error")

	_, err = os.Lstat(destination.String())
	testutils.RequireHasError(t, err, "expecting error on destination stat")
}

func TestFileSystemLinkerReadLink(t *testing.T) {
	destFolder, err := os.MkdirTemp("", "testreadlink")
	testutils.RequireNoError(t, err, "can't create temp folder %v", destFolder)
	defer os.Remove(destFolder)

	file := internal.Path("file-1.txt")
	source := internal.Path("testdata/package1").Join(file)
	destination := internal.Path(destFolder).Join(file)

	fs := internal.NewFileSystemLinker()
	err = fs.Link(source, destination)
	testutils.RequireNoError(t, err, "unexpected linking error")

	actual, err := fs.ReadLink(destination)
	testutils.RequireNoError(t, err, "unexpected readlink error")

	internaltest.AssertEqualPath(t, source, actual, "invalid source value")
}

func TestFileSystemLinkerReadLinkFileNotFound(t *testing.T) {
	destination := internal.Path("/a/wrong/path/to/file.txt")

	fs := internal.NewFileSystemLinker()
	_, err := fs.ReadLink(destination)
	testutils.RequireHasError(t, err, "unexpected readlink error")
	testutils.AssertErrorIs(t, internal.ErrLinkNotExist, err, "invalid readlink type")
}
