#!/bin/bash

make && LD_LIBRARY_PATH=../../target/debug ./main ./seed.txt
