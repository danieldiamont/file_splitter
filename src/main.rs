use clap::Parser;
use num_integer::div_rem;
use std::{
    fs::File,
    io::{Read, Write},
};

const CRC_POLYNOMIAL: u32 = 0xEDB88320;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,

    #[arg(long, default_value_t = 2048)]
    chunk_size: usize,

    #[arg(long, action)]
    compute_crcs: bool,

    #[arg(long)]
    compare_with: Option<String>,
}

fn crc32_file(file_name: String) -> u32 {
    let mut file = File::open(file_name).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    return crc32(&mut buffer);
}

fn crc32(buffer: &mut Vec<u8>) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;

    for &byte in buffer.iter() {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ CRC_POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
    }
    return crc ^ 0xFFFFFFFF;
}

fn get_part_name(
    file_name: String,
    part_num: usize,
    chunk_size: usize,
    file_size: usize,
) -> String {
    let num_chunks = calculate_num_chunks(file_size, chunk_size);
    let width = count_digits(num_chunks);
    return format!("{}.{:0width$}", file_name, part_num, width = width);
}

fn count_digits(num: usize) -> usize {
    let mut count = 0;
    let mut x = num;
    while x > 0 {
        count += 1;
        x /= 10;
    }
    return count;
}

fn calculate_num_chunks(file_size: usize, chunk_size: usize) -> usize {
    let (q, r) = div_rem(file_size, chunk_size);
    return q + ((r > 0) as usize);
}

fn process_chunk(buf: &mut Vec<u8>, num_bytes: usize, file_name: String, compute_crcs: bool) {
    let mut part_file = File::create(file_name.clone()).unwrap();
    let _ = part_file.write_all(&buf[0..num_bytes]);

    if compute_crcs {
        let crc = crc32_file(file_name.clone());
        println!(
            "Processing chunk #: {}\t-- size: {} (bytes) -- crc32 = {}",
            file_name.clone(),
            num_bytes,
            crc
        );
    } else {
        println!(
            "Processing chunk #: {}\t-- size: {} (bytes)",
            file_name.clone(),
            num_bytes
        );
    }
}

fn split_file(file_name: String, chunk_size: usize, compute_crcs: bool) {
    assert!(chunk_size > 0);
    println!("filename: {}\t chunk_size: {}", file_name, chunk_size);

    let mut file = File::open(file_name.clone()).unwrap();
    let mut buffer = vec![0; chunk_size];
    let mut _part_number = 0;

    let filesize = file.metadata().unwrap().len();

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let part_file_name = get_part_name(
                    file_name.clone(),
                    _part_number,
                    chunk_size,
                    filesize as usize,
                );
                process_chunk(&mut buffer, n, part_file_name, compute_crcs);
                _part_number += 1;
            }
            Err(e) => panic!("ERROR: {:?}", e),
        }
    }
}

fn main() {
    let args = Args::parse();
    split_file(args.file_name, args.chunk_size, args.compute_crcs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_empty_buffer() {
        let mut buf: Vec<u8> = Vec::new();

        assert_eq!(crc32(&mut buf), 0);
    }

    #[test]
    fn crc32_single_byte_buffer() {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(b'a');

        assert_eq!(crc32(&mut buf), 3904355907);
    }

    #[test]
    fn crc32_known_string() {
        let mut buf: Vec<u8> = Vec::from("123456789".as_bytes());

        assert_eq!(crc32(&mut buf), 3421780262);
    }
}
