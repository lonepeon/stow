package internal_test

import (
	"strings"
	"testing"

	"github.com/lonepeon/golib/testutils"
	"github.com/lonepeon/stow/internal"
)

func TestLoggerVerbosityAllow(t *testing.T) {
	tcs := map[string]struct {
		configuredVerbosity internal.LoggerVerbosity
		askedVerbosity      internal.LoggerVerbosity
		expected            bool
	}{
		"whenLoggerConfiguredWithErrorAndAskedError": {
			configuredVerbosity: internal.LoggerVerbosityError,
			askedVerbosity:      internal.LoggerVerbosityError,
			expected:            true,
		},
		"whenLoggerConfiguredWithErrorAndAskedInfo": {
			configuredVerbosity: internal.LoggerVerbosityError,
			askedVerbosity:      internal.LoggerVerbosityInfo,
			expected:            false,
		},
		"whenLoggerConfiguredWithErrorAndAskedDebug": {
			configuredVerbosity: internal.LoggerVerbosityError,
			askedVerbosity:      internal.LoggerVerbosityDebug,
			expected:            false,
		},
		"whenLoggerConfiguredWithInfoAndAskedError": {
			configuredVerbosity: internal.LoggerVerbosityInfo,
			askedVerbosity:      internal.LoggerVerbosityError,
			expected:            true,
		},
		"whenLoggerConfiguredWithInfoAndAskedInfo": {
			configuredVerbosity: internal.LoggerVerbosityInfo,
			askedVerbosity:      internal.LoggerVerbosityInfo,
			expected:            true,
		},
		"whenLoggerConfiguredWithInfoAndAskedDebug": {
			configuredVerbosity: internal.LoggerVerbosityInfo,
			askedVerbosity:      internal.LoggerVerbosityDebug,
			expected:            false,
		},
		"whenLoggerConfiguredWithDebugAndAskedError": {
			configuredVerbosity: internal.LoggerVerbosityDebug,
			askedVerbosity:      internal.LoggerVerbosityError,
			expected:            true,
		},
		"whenLoggerConfiguredWithDebugAndAskedInfo": {
			configuredVerbosity: internal.LoggerVerbosityDebug,
			askedVerbosity:      internal.LoggerVerbosityInfo,
			expected:            true,
		},
		"whenLoggerConfiguredWithDebugAndAskedDebug": {
			configuredVerbosity: internal.LoggerVerbosityDebug,
			askedVerbosity:      internal.LoggerVerbosityDebug,
			expected:            true,
		},
	}

	for name, tc := range tcs {
		t.Run(name, func(t *testing.T) {
			actual := tc.configuredVerbosity.Allow(tc.askedVerbosity)
			testutils.AssertEqualBool(t, tc.expected, actual, "invalid logging permission")
		})
	}
}

func TestLoggerWithVerbosity(t *testing.T) {
	var out strings.Builder
	logger := internal.NewLogger(&out)

	tcs := map[string]struct {
		verbosity  internal.LoggerVerbosity
		loggerFunc func(string, ...interface{})
		expected   bool
	}{
		"whenLoggerConfiguredWithErrorAndCallingDebugf": {
			verbosity:  internal.LoggerVerbosityError,
			loggerFunc: logger.Debugf,
			expected:   false,
		},
		"whenLoggerConfiguredWithErrorAndCallingInfof": {
			verbosity:  internal.LoggerVerbosityError,
			loggerFunc: logger.Infof,
			expected:   false,
		},
		"whenLoggerConfiguredWithErrorAndCallingErrorf": {
			verbosity:  internal.LoggerVerbosityError,
			loggerFunc: logger.Errorf,
			expected:   true,
		},
		"whenLoggerConfiguredWithInfoAndCallingDebugf": {
			verbosity:  internal.LoggerVerbosityInfo,
			loggerFunc: logger.Debugf,
			expected:   false,
		},
		"whenLoggerConfiguredWithInfoAndCallingInfof": {
			verbosity:  internal.LoggerVerbosityInfo,
			loggerFunc: logger.Infof,
			expected:   true,
		},
		"whenLoggerConfiguredWithInfoAndCallingErrorf": {
			verbosity:  internal.LoggerVerbosityInfo,
			loggerFunc: logger.Errorf,
			expected:   true,
		},
		"whenLoggerConfiguredWithDebugAndCallingDebugf": {
			verbosity:  internal.LoggerVerbosityDebug,
			loggerFunc: logger.Debugf,
			expected:   true,
		},
		"whenLoggerConfiguredWithDebugAndCallingInfof": {
			verbosity:  internal.LoggerVerbosityDebug,
			loggerFunc: logger.Infof,
			expected:   true,
		},
		"whenLoggerConfiguredWithDebugAndCallingErrorf": {
			verbosity:  internal.LoggerVerbosityDebug,
			loggerFunc: logger.Errorf,
			expected:   true,
		},
	}

	for name, tc := range tcs {
		t.Run(name, func(t *testing.T) {
			out.Reset()
			logger.SetVerbosity(tc.verbosity)
			tc.loggerFunc("log with %d variable", 1)
			actual := out.String() == "log with 1 variable\n"

			testutils.AssertEqualBool(t, tc.expected, actual, "invalid logging: %s", &out)
		})
	}
}
