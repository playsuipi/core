# Play Suipi Core

[![Rust](https://github.com/playsuipi/core/actions/workflows/rust.yml/badge.svg)](https://github.com/playsuipi/core/actions/workflows/rust.yml)

The core game logic that is at the center of all Play Suipi products.

## Quick Start

1. Install the Rust language. ([rust-lang.org](https://www.rust-lang.org/tools/install))
2. Run the demo game.

```bash
cargo run
```

Because Suipi randomly shuffles the deck before each game, you cannot play the
same game twice by default. You can get around this by specifying a *seed* to
use when shuffling. If you use the same *seed* twice, you will get the exact
same shuffle both times.

You can specify a *seed* for the game to use by passing an argument with the
path to a **seed file**. A seed file is just a plain text file with a number
from `0` to `255` on each line. There may be up to 32 lines in the file,
anything over that will be ignored, and anything less than that will be assumed
to be a zero. So a blank file would be a seed of 32 zeros.

```bash
touch ./seed.txt
cargo run ./seed.txt
```

## Compile for Mobile

Rust needs to install target plugins for each architecture we want to compile
for. We can do this using the `rustup` tool installed in the "Quick Start".

### Android

**Install Android targets:**

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

### iOS

**Install nightly rustup channel:**

This allows us to use the latest features in the Rust compiler.

```bash
rustup target install nightly
```

**Install iOS target:**

This allows us to compile Rust code for iOS devices.

```bash
rustup +nightly target add aarch64-apple-ios
```

**Install cargo lipo command:**

This command lets us build an iOS static library.

```bash
cargo install cargo-lipo
```

#### Extra

**Install cbindgen command:**

This tool lets us generate a C header file with bindings for our library.

> We don't need this unless we update the core library...

```bash
cargo install cbindgen
```

**Install macOS targets:**

This allows us to compile Rust code for iOS devices.

> For non-Apple Silicon devices we need `x86_64` support.

```bash
rustup +nightly target add aarch64-apple-ios x86_64-apple-ios
```

## Testing

Run the unit and integration tests.

```bash
cargo test
```

## Debugging

On Linux, you can run `cargo` commands inside of
[GDB](https://sourceware.org/gdb/) using the `./bin/debug.sh` script.

```bash
./bin/debug.sh test
```

### Reproducing Issues

There is also a `./bin/record.py` script that you can use to record your moves
as you go, and replay them if you need to restart.

```bash
python3 -m pip install --user pwntools
touch log.txt seed.txt
./bin/record.py
```

The `./bin/record.py` script will automatically save your moves to a log file,
which you can configure with the `LOG_PATH` argument. By default this is set to
`./log.txt`. The log file contains plain text Suipi annotations on each line.

```bash
echo '!1' >> log2.txt
echo '!1' >> log2.txt
./bin/record.py LOG_PATH=./log2.txt
```

To specify a **seed file** for your game, you can use the `SEED_PATH` argument.
By default this is set to `./seed.txt`.

```bash
touch ./seed2.txt
./bin/record.py SEED_PATH=./seed2.txt
```

The `./bin/record.py` script will automatically run *GDB* for you if you pass a
`GDB` argument.

> **Note:** By default the script will attempt to run the *GDB* process in a
> new [tmux](https://github.com/tmux/tmux/wiki) pane. If you are not using
> *tmux*, you may configure this behaviour by setting the `context.terminal`
> variable at the top of the script.
>
> * [pwntools
>   ContextType.terminal](https://docs.pwntools.com/en/stable/context.html#pwnlib.context.ContextType.terminal)

```bash
./bin/record.py GDB
```
