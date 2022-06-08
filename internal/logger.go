package internal

import (
	"fmt"
	"io"
)

type LoggerVerbosity struct {
	level int
}

func (v LoggerVerbosity) Allow(other LoggerVerbosity) bool {
	return v.level >= other.level
}

var (
	LoggerVerbosityError = LoggerVerbosity{level: 0}
	LoggerVerbosityInfo  = LoggerVerbosity{level: 1}
	LoggerVerbosityDebug = LoggerVerbosity{level: 2}
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
