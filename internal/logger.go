package internal

import (
	"fmt"
	"io"
)

type LoggerVerbosity struct {
	level int
	name  string
}

func NewLoggerVerbosity(level int) (LoggerVerbosity, error) {
	for _, verbosity := range loggerVerbosities {
		if verbosity.level == level {
			return verbosity, nil
		}
	}

	return LoggerVerbosityError, fmt.Errorf("invalid level %d", level)
}

func (v LoggerVerbosity) Allow(other LoggerVerbosity) bool {
	return v.level >= other.level
}

func (v LoggerVerbosity) String() string {
	return v.name
}

var (
	LoggerVerbosityError = LoggerVerbosity{level: 0, name: "error"}
	LoggerVerbosityInfo  = LoggerVerbosity{level: 1, name: "info"}
	LoggerVerbosityDebug = LoggerVerbosity{level: 2, name: "debug"}

	loggerVerbosities = []LoggerVerbosity{
		LoggerVerbosityError,
		LoggerVerbosityInfo,
		LoggerVerbosityDebug,
	}
)

type Logger struct {
	verbosity LoggerVerbosity
	out       io.Writer
}

func NewLogger(out io.Writer) *Logger {
	return &Logger{out: out, verbosity: LoggerVerbosityError}
}

func (l *Logger) SetVerbosity(verbosity LoggerVerbosity) {
	l.verbosity = verbosity
}

func (l *Logger) Debugf(format string, values ...interface{}) {
	l.printf(LoggerVerbosityDebug, format, values...)
}

func (l *Logger) Infof(format string, values ...interface{}) {
	l.printf(LoggerVerbosityInfo, format, values...)
}

func (l *Logger) Errorf(format string, values ...interface{}) {
	l.printf(LoggerVerbosityError, format, values...)
}

func (l *Logger) SetOutput(out io.Writer) {
	l.out = out
}

func (l *Logger) printf(verbosity LoggerVerbosity, format string, values ...interface{}) {
	if !l.verbosity.Allow(verbosity) {
		return
	}

	fmt.Fprintln(l.out, fmt.Sprintf(format, values...))
}
