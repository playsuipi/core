.DEFAULT_GOAL := help
PROJECTNAME=$(shell basename "$(PWD)")
SOURCES=$(sort $(wildcard ./src/*.rs ./src/**/*.rs))

ANDROID_NDK_HOME=$(HOME)/Library/Android/Sdk/ndk/26.1.10909125

OS_NAME=$(shell uname | tr '[:upper:]' '[:lower:]')
PATH := $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin:$(PATH)

ANDROID_AARCH64_LINKER=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/aarch64-linux-android29-clang
ANDROID_ARMV7_LINKER=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/armv7a-linux-androideabi29-clang
ANDROID_I686_LINKER=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/i686-linux-android29-clang
ANDROID_X86_64_LINKER=$(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/bin/x86_64-linux-android29-clang

SHELL := /bin/bash

# ##############################################################################
# # GENERAL
# ##############################################################################

.PHONY: help
help: Makefile
	@echo
	@echo " Available actions in "$(PROJECTNAME)":"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo

## init: Install missing dependencies.
.PHONY: init
init:
	@if [ $$(uname) == "Darwin" ] ; then \
		rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios \
		rustup target add aarch64-apple-darwin x86_64-apple-darwin \
		; fi
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
	cargo install cbindgen

## :

# ##############################################################################
# # RECIPES
# ##############################################################################

## all: Compile iOS, Android and bindings targets
all: ios macos android bindings

## ios: Compile the iOS universal library
ios: target/aarch64-apple-ios/release/libplaysuipi_core.a target/aarch64-apple-ios-sim/release/libplaysuipi_core.a target/x86_64-apple-ios/release/libplaysuipi_core.a

target/aarch64-apple-ios/release/libplaysuipi_core.a: $(SOURCES)
	@if [ $$(uname) == "Darwin" ] ; then \
		cargo build --target aarch64-apple-ios --release ; \
		else echo "Skipping iOS compilation on $$(uname)" ; \
	fi
	@echo "[DONE] $@"

target/aarch64-apple-ios-sim/release/libplaysuipi_core.a: $(SOURCES)
	@if [ $$(uname) == "Darwin" ] ; then \
		cargo build --target aarch64-apple-ios-sim --release ; \
		else echo "Skipping iOS compilation on $$(uname)" ; \
	fi
	@echo "[DONE] $@"

target/x86_64-apple-ios/release/libplaysuipi_core.a: $(SOURCES)
	@if [ $$(uname) == "Darwin" ] ; then \
		cargo build --target x86_64-apple-ios --release ; \
		else echo "Skipping iOS compilation on $$(uname)" ; \
	fi
	@echo "[DONE] $@"



## macos: Compile the macOS libraries
macos: target/x86_64-apple-darwin/release/libplaysuipi_core.a target/aarch64-apple-darwin/release/libplaysuipi_core.a

target/aarch64-apple-darwin/release/libplaysuipi_core.a: $(SOURCES)
	@if [ $$(uname) == "Darwin" ] ; then \
		cargo build --target aarch64-apple-darwin --release ; \
		else echo "Skipping macOS compilation on $$(uname)" ; \
	fi
	@echo "[DONE] $@"

target/x86_64-apple-darwin/release/libplaysuipi_core.a: $(SOURCES)
	@if [ $$(uname) == "Darwin" ] ; then \
		cargo build --target x86_64-apple-darwin --release ; \
		else echo "Skipping macOS compilation on $$(uname)" ; \
	fi
	@echo "[DONE] $@"

## android: Compile the android targets (arm64, armv7 and i686)
android: target/aarch64-linux-android/release/libplaysuipi_core.so target/armv7-linux-androideabi/release/libplaysuipi_core.so target/i686-linux-android/release/libplaysuipi_core.so target/x86_64-linux-android/release/libplaysuipi_core.so

target/aarch64-linux-android/release/libplaysuipi_core.so: $(SOURCES) ndk-home
	CC_aarch64_linux_android=$(ANDROID_AARCH64_LINKER) \
	CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$(ANDROID_AARCH64_LINKER) \
		cargo build --target aarch64-linux-android --release
	@echo "[DONE] $@"

target/armv7-linux-androideabi/release/libplaysuipi_core.so: $(SOURCES) ndk-home
	CC_armv7_linux_androideabi=$(ANDROID_ARMV7_LINKER) \
	CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$(ANDROID_ARMV7_LINKER) \
		cargo build --target armv7-linux-androideabi --release
	@echo "[DONE] $@"

target/i686-linux-android/release/libplaysuipi_core.so: $(SOURCES) ndk-home
	CC_i686_linux_android=$(ANDROID_I686_LINKER) \
	CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$(ANDROID_I686_LINKER) \
		cargo  build --target i686-linux-android --release
	@echo "[DONE] $@"

target/x86_64-linux-android/release/libplaysuipi_core.so: $(SOURCES) ndk-home
	CC_x86_64_linux_android=$(ANDROID_X86_64_LINKER) \
	CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$(ANDROID_X86_64_LINKER) \
		cargo build --target x86_64-linux-android --release
	@echo "[DONE] $@"
		
.PHONY: ndk-home
ndk-home:
	@if [ ! -d "${ANDROID_NDK_HOME}" ] ; then \
		echo "Error: Please, set the ANDROID_NDK_HOME env variable to point to your NDK folder" ; \
		exit 1 ; \
	fi

## bindings: Generate the .h file for iOS
bindings: target/playsuipi_core.h

target/playsuipi_core.h:
	cbindgen --crate playsuipi_core -c cbindgen.toml | uniq > $@
	@echo "[DONE] $@"

## :

# ##############################################################################
# # OTHER
# ##############################################################################

## clean:
.PHONY: clean
clean:
	cargo clean
	rm -f target/playsuipi_core.h

## test:
.PHONY: test
test:
	cargo test
