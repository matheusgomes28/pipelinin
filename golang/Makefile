# Go parameters
GOCMD=go
BUILD_DIR=./build
APP_DIR=./cmd/app

# Name of the binary
BINARY_NAME=app

all: build test

build:
	GIN_MODE=release $(GOCMD) build -ldflags "-s" -v -o $(BUILD_DIR)/$(BINARY_NAME) $(APP_DIR)

test:
	$(GOCMD) test -v ./...

clean:
	$(GOCMD) clean
	rm -rf $(BUILD_DIR)

.PHONY: all build test clean
