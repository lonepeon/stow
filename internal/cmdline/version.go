package cmdline

import (
	"fmt"
	"io"

	"github.com/lonepeon/stow/internal/build"
)

func Version(out io.Writer) {
	fmt.Fprintln(out, build.GetVersion())
}
