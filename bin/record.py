#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# pylint: disable=line-too-long,missing-module-docstring,wildcard-import,redefined-builtin,unused-wildcard-import,pointless-string-statement,invalid-name,undefined-variable,missing-function-docstring

import sys

# Dependencies:
#   python3 -m pip install pwntools
from pwn import *


binary = args.BINARY or "./target/debug/playsuipi_core"
elf = ELF(binary)


# Context Settings
context.terminal = ["tmux", "splitw", "-h"]
context.log_level = args.LOGS or "info"
context.binary = elf


seed_path = args.SEED_PATH or "seed.txt"
log_path = args.LOG_PATH or "log.txt"
INPUT_YOUR_MOVE = b"> Input your move below:\n"


count = 0


def check_end():
    global count
    if count >= 8 * 12:
        for _ in range(7):
            print(io.recvline().decode("utf-8").replace("\n", ""))
        log.info(f"Log saved to: {log_path}")
        io.close()
        sys.exit()


def load():
    global count
    with open(log_path, "r") as f:
        for x in f:
            print(io.recvuntil(INPUT_YOUR_MOVE).decode("utf-8"))
            log.info(x)
            print()
            print()
            io.sendline(x.strip().encode())
            count += 1
    check_end()


def store():
    while True:
        print(io.recvuntil(INPUT_YOUR_MOVE).decode("utf-8"))
        x = input()
        print()
        print()
        io.sendline(x.strip().encode())
        with open(log_path, "a") as f:
            f.write("\n" + x.strip())


gdbscript = """
""".format(
    **locals()
)


def local_io():
    if args.GDB:
        return gdb.debug([binary, seed_path], gdbscript=gdbscript)
    return process([binary, seed_path])


if __name__ == "__main__":
    io = local_io()
    load()
    store()
