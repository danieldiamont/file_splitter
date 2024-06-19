use clap::Parser;
use num_integer::div_rem;
use std::path::PathBuf;
use std::{
    fs::File,
    io::{Read, Write},
};

const CRC_POLYNOMIAL: u32 = 0xEDB88320;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: PathBuf,

    #[arg(long, default_value_t = 2048)]
    chunk_size: usize,

    #[arg(long)]
    compare_dir: Option<PathBuf>,
}

fn crc32_file(file_path: PathBuf) -> u32 {
    let mut file = File::open(file_path.clone())
        .unwrap_or_else(|_| panic!("Tried to open {:#?}", file_path.clone()));
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    crc32(&mut buffer)
}

fn crc32(buffer: &mut [u8]) -> u32 {
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
    crc ^ 0xFFFFFFFF
}

fn get_part_name(
    file_path: String,
    part_num: usize,
    chunk_size: usize,
    file_size: usize,
) -> String {
    let num_chunks = calculate_num_chunks(file_size, chunk_size);
    let width = count_digits(num_chunks);
    format!("{}.{:0width$}", file_path, part_num, width = width)
}

fn count_digits(num: usize) -> usize {
    let mut count = 0;
    let mut x = num;
    while x > 0 {
        count += 1;
        x /= 10;
    }

    count
}

fn calculate_num_chunks(file_size: usize, chunk_size: usize) -> usize {
    let (q, r) = div_rem(file_size, chunk_size);

    q + ((r > 0) as usize)
}

fn process_chunk(
    buf: &mut [u8],
    num_bytes: usize,
    file_path: PathBuf,
    compare: Option<PathBuf>,
) {
    let mut part_file = File::create(file_path.clone()).unwrap();
    let _ = part_file.write_all(&buf[0..num_bytes]);

    let crc = crc32_file(file_path.clone());
    println!(
        "Processing chunk #: {:#?}\t-- size: {} (bytes) -- crc32 = 0x{:08X}",
        file_path.clone(),
        num_bytes,
        crc
    );

    let crc_comp: Option<u32> = compare.clone().map(crc32_file);

    if let Some(crc_comp_val) = crc_comp {
        assert_eq!(
            crc,
            crc_comp_val,
            "CRC Comparison {:#?} crc = 0x{:08X};\t{:#?} crc = 0x{:08X}",
            file_path,
            crc,
            compare.unwrap(),
            crc_comp_val
        );
    }
}

fn split_file(file_path: PathBuf, chunk_size: usize, compare_dir: Option<PathBuf>) {
    assert!(chunk_size > 0);
    println!("filename: {:#?}\t chunk_size: {}", file_path, chunk_size);

    let mut file = File::open(file_path.clone()).unwrap();
    let mut buffer = vec![0; chunk_size];
    let mut _part_number = 0;

    let filesize = file.metadata().unwrap().len();
    let filename = file_path.file_name().unwrap();
    let parent_dir = file_path.parent().unwrap();

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let part_file_name = get_part_name(
                    filename.to_string_lossy().to_string(),
                    _part_number,
                    chunk_size,
                    filesize as usize,
                );

                let mut part_file_path = PathBuf::new();
                part_file_path.push(parent_dir);
                part_file_path.push(part_file_name.clone());

                let comp: Option<PathBuf> = compare_dir
                    .as_ref()
                    .map(|dir| dir.join(part_file_name.clone()));

                process_chunk(&mut buffer, n, part_file_path, comp);

                _part_number += 1;
            }
            Err(e) => panic!("ERROR: {:?}", e),
        }
    }
}

fn main() {
    let args = Args::parse();
    split_file(args.file_path, args.chunk_size, args.compare_dir);
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
