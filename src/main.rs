use std::{fs::File, io::{Read, Write}};
use clap::Parser;

const CRC_POLYNOMIAL : u32 = 0x04C11DB7;

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
    compare_with: Option<String>

}

fn crc32(file_name: String, num_bytes: usize) -> u32 {
    let mut crc : u32 = 0xFFFFFFFF;

    let mut file = File::open(file_name).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    for i in 0..num_bytes {
        let ch : u32 = buffer[i].into(); 
        for _ in 0..8 {
            let tmp = (ch ^ crc) & 1;
            crc >>= 1;
            if tmp != 0 {
                crc ^= CRC_POLYNOMIAL;
            }
            crc >>= 1;
        }
    }
    return !crc;
}

fn get_part_name(file_name: String, part_num: usize) -> String {
    return format!("{}.{:03}", file_name, part_num);
}

fn process_chunk(buf: &mut Vec<u8>, num_bytes: usize, file_name: String, part_num: usize, compute_crcs: bool) {
    let part_file_name = get_part_name(file_name, part_num);
    let mut part_file = File::create(part_file_name.clone()).unwrap();
    let _ = part_file.write_all(&buf[0..num_bytes]);

    if compute_crcs {
        let crc = crc32(part_file_name.clone(), num_bytes);
        println!("Processing chunk #: {}\t-- size: {} (bytes) -- crc32 = {}", part_num, num_bytes, crc);
    } else {
        println!("Processing chunk #: {}\t-- size: {} (bytes)", part_num, num_bytes);
    }
}

fn split_file(file_name: String, chunk_size: usize, compute_crcs: bool) {
    assert!(chunk_size > 0);
    println!("filename: {}\t chunk_size: {}", file_name, chunk_size);

    let mut file = File::open(file_name.clone()).unwrap();
    let mut buffer = vec![0; chunk_size];
    let mut _part_number = 0;

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                process_chunk(&mut buffer, n, file_name.clone(), _part_number, compute_crcs);
                _part_number += 1;
            },
            Err(e) => panic!("ERROR: {:?}", e),
        }
    }
}

fn main() {
    let args = Args::parse();
    split_file(args.file_name, args.chunk_size, args.compute_crcs);
}
