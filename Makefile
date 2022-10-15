BUILD_FOLDER = target
BUILD_OPTIONS =

OS := linux
ARCH := amd64
BINARY_NAME := "stow"
FULL_BINARY_NAME := $(BINARY_NAME)-$(OS)-$(ARCH)

PROJECT_USERNAME := lonepeon
PROJECT_REPOSITORY := stow

GO_BIN := go
GIT_BIN := git

VERSION := $(shell date -u +'%Y-%m-%dT%H:%M:%SZ')
GIT_COMMIT := $(shell $(GIT_BIN) rev-parse HEAD)
GIT_BRANCH := $(shell $(GIT_BIN) branch --no-color | awk '/^\* / { print $$2 }')
GIT_STATE := $(shell if [ -z "$(shell $(GIT_BIN) status --short)" ]; then echo clean; else echo dirty; fi)

release: compile

deps:
	$(GO_BIN) install github.com/golangci/golangci-lint/cmd/golangci-lint@v1.46.2

compile:
	@echo "+$@"
	@touch internal/build/version.go
	@GOOS=$(OS) GOARCH=$(ARCH) $(GO_BIN) build $(BUILD_OPTIONS) \
		-ldflags \
			 "-X github.com/lonepeon/stow/internal/build.branch=$(GIT_BRANCH) \
		 	  -X github.com/lonepeon/stow/internal/build.commit=$(GIT_COMMIT) \
			  -X github.com/lonepeon/stow/internal/build.state=$(GIT_STATE) \
			  -X github.com/lonepeon/stow/internal/build.version=$(VERSION)" \
		-o $(BUILD_FOLDER)/$(FULL_BINARY_NAME)

test: test-unit test-lint

test-unit:
	@echo "+$@"
	@$(GO_BIN) test -cover ./...

.PHONY: test-lint
test-lint:
	@echo "+$@"
	@golangci-lint run
