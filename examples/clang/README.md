# C Reference Implementation

This is a simple CLI based Suipi game written in C. It is a reference
implementation for using the Play Suipi Core library.

## Play Suipi

```bash
cd ./core
cargo build && make bindings
cd ./examples/clang
./play.sh
```

## Testing

Record your moves and replay them for quick in-game testing.

```bash
cd ./core
cargo build && make bindings
cd ./examples/clang
make
touch log.txt seed.txt
LD_LIBRARY_PATH="../../target/debug" ../../bin/record.py BINARY="./main"
```
