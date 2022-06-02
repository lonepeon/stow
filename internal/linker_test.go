package internal_test

import (
	"os"
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
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
			source:      internal.Path("testdata/bundle1/file-1.txt"),
			destination: destFolderPath.Join(internal.Path("file-1.txt")),
		},
		"with-subfolder": {
			source:      internal.Path("testdata/bundle1/subfolder/file-3.txt"),
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

	source := internal.Path("testdata/bundle1/file-1.txt")
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
