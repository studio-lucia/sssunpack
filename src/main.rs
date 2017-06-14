use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

extern crate clap;
use clap::{Arg, App};

// Size of a Mode-1 CD-ROM sector, in bytes
const SECTOR_LENGTH : usize = 2048;

struct FileEntry {
    name: String,
    start: u16,
    end: u16,
    length: u16,
}

fn uint16_from_bytes(bytes : [u8; 2]) -> u16 {
    return ((bytes[0] as u16) << 8) + bytes[1] as u16;
}

fn parse_file_listing(data : &[u8]) -> Option<FileEntry> {
    // Filename can't begin with nul bytes, so this indicates
    // dummy data.
    if data[0] == 0x0 {
        return None;
    }

    let mut filename : Vec<u8> = vec![];
    filename.extend_from_slice(&data[0..11]);

    return Some(FileEntry {
        name: String::from_utf8(filename).unwrap(),
        start: uint16_from_bytes([data[14], data[15]]),
        end: uint16_from_bytes([data[18], data[19]]),
        length: uint16_from_bytes([data[22], data[23]]),
    });
}

fn do_stuff(input : String, target : String) -> Result<(), String> {
    let input_path = Path::new(&input);
    let target_path = Path::new(&target);

    if !input_path.exists() {
        return Err(format!("The specified input file ({}) does not exist!", input));
    } else if !input_path.is_file() {
        return Err(format!("The specified input file ({}) is not a file!", input));
    }

    if !target_path.is_dir() {
        return Err(format!("The specified target directory ({}) is invalid!", target));
    }

    let input_file = File::open(&input_path).unwrap();
    let mut buf_reader = BufReader::new(input_file);
    let mut data : Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut data);

    println!("{}", input_path.to_string_lossy());
    return Ok(());
}

fn main() {
    let matches = App::new("sssunpack")
                          .version("0.1.0")
                          .author("Misty De Meo")
                          .about("Unpack Lunar: Silver Star Story Complete data files")
                          .arg(Arg::with_name("input")
                              .help("The file to unpack")
                              .required(true)
                              .index(1))
                          .arg(Arg::with_name("target")
                              .help("Directory into which unpacked files should be saved")
                              .index(2))
                          .get_matches();
    let input = matches.value_of("input").unwrap().to_string();
    let target = matches.value_of("target").unwrap_or(".").to_string();

    exit(match do_stuff(input, target) {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        },
    });
}
