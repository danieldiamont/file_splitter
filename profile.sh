#!/bin/bash

FILE="Cargo.toml"
DEBUG=$(grep "profile.release" Cargo.toml | wc -l)
if [ $DEBUG -eq 0 ]; then
    echo "" >> $FILE
    echo "[profile.release]" >> $FILE
    echo "debug = true" >> $FILE
fi

rm -f callgrind.out.*
cargo build --release
valgrind --tool=callgrind --collect-jumps=yes --dump-instr=yes ./target/release/file_splitter --file-path assets/large.txt --chunk-size 204800 --compare-dir tmp/
ls -lht callgrind.out.* | head -n 1 | kcachegrind
