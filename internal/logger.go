package internal

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
