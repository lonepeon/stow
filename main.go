package main

import (
	"flag"
	"fmt"
	"os"

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
	ShowVersion bool
}

func run() error {
	var flags Flags
	flagset := flag.NewFlagSet("stow", flag.ExitOnError)

	flagset.BoolVar(&flags.ShowVersion, "version", false, "display current commandline version")

	err := flagset.Parse(os.Args[1:])
	if err != nil {
		return err
	}

	if flags.ShowVersion {
		cmdline.Version(os.Stdout)
		return nil
	}

	return nil
}
