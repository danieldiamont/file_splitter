# File Splitter with CRC32 Verification

This Rust program splits a file into smaller chunks of a specified size and optionally compares each chunk's CRC32 checksum with a corresponding file in another directory.

## Features

- Splits a file into smaller chunks.
- Calculates the CRC32 checksum for each chunk.
- Optionally compares the CRC32 checksum of each chunk with a corresponding file in another directory.
- Provides useful output messages for each processed chunk.

## Usage

To run this program, you need to have Rust installed. You can then compile and run the program using `cargo`.

### Command Line Arguments

- `-f, --file-path <FILE_PATH>`: Path to the file to be split.
- `--chunk-size <CHUNK_SIZE>`: Size of each chunk in bytes (default: 2048).
- `--compare-dir <COMPARE_DIR>`: Optional directory path to compare chunks' CRC32 checksums.

### Example

```sh
cargo run --release -- --file-path /path/to/your/file --chunk-size 2048 --compare-dir /path/to/compare/dir
```

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
