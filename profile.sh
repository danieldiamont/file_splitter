#!/bin/bash

cargo build --release
valgrind --tool=callgrind --collect-jumps=yes --dump-instr=yes ./target/release/file_splitter --file-path assets/large.txt --chunk-size 204800 --compare-dir tmp/
ls -lht callgrind.out.* | head -n 1 | kcachegrind
