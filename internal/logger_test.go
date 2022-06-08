package internal_test

import (
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
