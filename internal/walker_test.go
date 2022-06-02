package internal_test

import (
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
	"github.com/lonepeon/stow/internal/internaltest"
)

func TestWalkFileSystem(t *testing.T) {
	root := internal.Path("testdata/package1")
	var paths []internal.Path

	err := internal.WalkFileSystem(root, func(path internal.Path) {
		paths = append(paths, path)
	})
	testutils.RequireNoError(t, err, "unexpected error")

	expectedFiles := []internal.Path{
		internal.Path("file-1.txt"),
		internal.Path("file-2.txt"),
		internal.Path("subfolder/file-3.txt"),
	}
	testutils.RequireEqualInt(t, len(expectedFiles), len(paths), "invalid number of files")
	for i := range expectedFiles {
		internaltest.AssertEqualPath(t, expectedFiles[i], paths[i], "invalid path at index %d", i)
	}
}
