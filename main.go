package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/lonepeon/stow/internal"
	"github.com/lonepeon/stow/internal/cmdline"
)

func main() {
	err := run()
	if err != nil {
		fmt.Fprintln(os.Stderr, err.Error())
		os.Exit(1)
	}
}

type Flags struct {
	showVersion     bool
	stowDirectory   string
	dryRun          bool
	verbose         int
	targetDirectory string
	unstow          bool
}

func run() error {
	var flags Flags
	flagset := flag.NewFlagSet("stow", flag.ExitOnError)
	flagset.Usage = func() {
		var options strings.Builder
		oldOutput := flagset.Output()

		flagset.SetOutput(&options)
		flagset.PrintDefaults()
		flagset.SetOutput(oldOutput)

		fmt.Fprintf(flagset.Output(), helptext, flagset.Name(), options.String())
	}

	flagset.IntVar(&flags.verbose, "v", 0, "Write verbose output to STDERR. From 0 (quiet) to 2 (chatty)")
	flagset.BoolVar(&flags.showVersion, "V", false, "Display current commandline version")
	flagset.BoolVar(&flags.dryRun, "n", false, "Do not execute any operations that modify the filesystem, only print commands")
	flagset.StringVar(&flags.stowDirectory, "d", "", "Set the stow directory instead of using the STOW_DIR environment variable or the current directory")
	flagset.StringVar(&flags.targetDirectory, "t", "", "Set the target directory instead of using the parent of the stow directory")
	flagset.BoolVar(&flags.unstow, "D", false, "Delete the packages from the target directory instead of installing them")

	err := flagset.Parse(os.Args[1:])
	if err != nil {
		return err
	}

	if flags.showVersion {
		cmdline.Version(os.Stdout)
		return nil
	}
	if flags.dryRun {
		cmdline.DryRunExecutor(os.Stdout)
	}

	verbosity, err := internal.NewLoggerVerbosity(flags.verbose)
	if err != nil {
		return fmt.Errorf("invalid verbosity value: %v", err)
	}
	logger := internal.NewLogger(os.Stderr)
	logger.SetVerbosity(verbosity)
	cmdline.VerboseMode(logger)

	logger.Debugf("dry-run mode enabled: %t", flags.dryRun)

	defaultStowPath := getDirectoryFromEnvWithDefault("STOW_DIR", ".")
	logger.Debugf("default stow directory: '%s'", defaultStowPath)

	stowPath, err := getDirectoryWithDefault(flags.stowDirectory, defaultStowPath)
	if err != nil {
		return fmt.Errorf("can't build stow directory: %v", err)
	}

	logger.Infof("stow directory set to '%s'", stowPath)

	defaultTargetPath := stowPath.Join(internal.Path("..")).String()
	logger.Debugf("default target directory: '%s'", defaultTargetPath)

	targetPath, err := getDirectoryWithDefault(flags.targetDirectory, defaultTargetPath)
	if err != nil {
		return fmt.Errorf("can't build target directory: %v", err)
	}

	logger.Infof("target directory set to '%s'", targetPath)

	actionName := "stow"
	action := cmdline.Stow
	if flags.unstow {
		actionName = "unstow"
		action = cmdline.Unstow
	}

	for _, pkgName := range flagset.Args() {
		logger.Infof("start to %s package '%s'", actionName, pkgName)
		err := action(stowPath, targetPath, pkgName)
		if err != nil {
			return fmt.Errorf("failed to %s package '%s': %v", actionName, pkgName, err)
		}
		logger.Infof("finished to %s package '%s'", actionName, pkgName)
	}

	return nil
}

func getDirectoryWithDefault(override string, defaultValue string) (internal.Path, error) {
	value := override
	if value == "" {
		value = defaultValue
	}

	absolutePath, err := filepath.Abs(filepath.Clean(os.ExpandEnv(value)))
	if err != nil {
		return internal.Path(absolutePath), fmt.Errorf("can't expand path '%s': %v", value, err)
	}

	return internal.Path(absolutePath), nil
}

func getDirectoryFromEnvWithDefault(envName string, defaultValue string) string {
	value := os.Getenv(envName)
	if value == "" {
		value = defaultValue
	}

	return value
}

const helptext = `%[1]s [flags...] package...

The command line is in charge of symlinking files from different directory in another place.

It helps manage packages individually while still being able to install them in a shared file tree.

Flags:

%[2]s
Arguments:

  package
      a list of at least one package name to install. A package name is a folder present at the root oof the stow directory
`
