package internaltest

import (
	"fmt"
	"testing"

	"github.com/lonepeon/stow/internal"
)

func AssertEqualLoggerVerbosity(t *testing.T, want internal.LoggerVerbosity, got internal.LoggerVerbosity, format string, args ...interface{}) {
	t.Helper()

	if want != got {
		t.Errorf("%s\nwant: %s\ngot:  %s\n", fmt.Sprintf(format, args...), want, got)
	}
}
