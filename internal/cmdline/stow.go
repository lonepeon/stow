package cmdline

import (
	"io"

	"github.com/lonepeon/stow/internal"
)

var (
	executor internal.FileSystemExecutor = internal.NewLocalFileSystemExecutor()
)

func DryRunExecutor(out io.Writer) {
	executor = internal.NewDryRunFileSystemExecutor(out)
}

func VerboseMode(logger *internal.Logger) {
	executor = internal.NewVerboseFileSystemExecutor(logger, executor)
}

func Stow(stowPath internal.Path, targetPath internal.Path, pkgName string) error {
	pkg, err := internal.BuildPackage(stowPath, pkgName)
	if err != nil {
		return err
	}

	linker := internal.NewFileSystemLinker(executor)
	target := internal.NewTargetWithLinker(targetPath, linker)

	return target.Stow(stowPath, pkg)
}

func Unstow(stowPath internal.Path, targetPath internal.Path, pkgName string) error {
	pkg, err := internal.BuildPackage(stowPath, pkgName)
	if err != nil {
		return err
	}

	linker := internal.NewFileSystemLinker(executor)
	target := internal.NewTargetWithLinker(targetPath, linker)

	return target.Unstow(stowPath, pkg)
}
