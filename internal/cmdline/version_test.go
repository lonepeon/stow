package cmdline_test

import (
	"strings"
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal/cmdline"
)

func TestVersion(t *testing.T) {
	var out strings.Builder

	cmdline.Version(&out)

	testutils.AssertContainsString(t, "HEAD", out.String(), "invalid version parameter")
	testutils.AssertContainsString(t, "nobranch", out.String(), "invalid version parameter")
	testutils.AssertContainsString(t, "dirty", out.String(), "invalid version parameter")
	testutils.AssertContainsString(t, "unknown", out.String(), "invalid version parameter")
}
