package internaltest

import (
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
)

func AssertEqualPath(t *testing.T, want, got internal.Path, pattern string, args ...interface{}) {
	t.Helper()

	testutils.AssertEqualString(t, want.String(), got.String(), pattern, args...)
}
