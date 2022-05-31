package internal_test

import (
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
	"github.com/lonepeon/stow/internal/internaltest"
)

func TestPathString(t *testing.T) {
	filepath := "a/path/to/file"
	path := internal.Path(filepath)
	testutils.AssertEqualString(t, filepath, path.String(), "invalid Stringer implementation")
}

func TestPathJoin(t *testing.T) {
	tcs := map[string]struct {
		path     internal.Path
		other    internal.Path
		expected internal.Path
	}{
		"emptyOther": {
			path:     internal.Path("a/folder"),
			other:    internal.Path(""),
			expected: internal.Path("a/folder"),
		},
		"oneFolderDepthOther": {
			path:     internal.Path("a/folder"),
			other:    internal.Path("file"),
			expected: internal.Path("a/folder/file"),
		},
		"multiFolderDepthOther": {
			path:     internal.Path("a/folder"),
			other:    internal.Path("path/to/file"),
			expected: internal.Path("a/folder/path/to/file"),
		},
		"oneFolderDepthPath": {
			path:     internal.Path("folder"),
			other:    internal.Path("path/to/file"),
			expected: internal.Path("folder/path/to/file"),
		},
		"multiFolderDepthPath": {
			path:     internal.Path("a/folder"),
			other:    internal.Path("path/to/file"),
			expected: internal.Path("a/folder/path/to/file"),
		},
		"tralingSlashes": {
			path:     internal.Path("a/folder/"),
			other:    internal.Path("path/to/file"),
			expected: internal.Path("a/folder/path/to/file"),
		},
	}

	for name, tc := range tcs {
		t.Run(name, func(t *testing.T) {
			actual := tc.path.Join(tc.other)
			internaltest.AssertEqualPath(t, tc.expected, actual, "invalid path")
		})
	}
}
