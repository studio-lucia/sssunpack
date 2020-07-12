use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use sss_unpack::consts::SECTOR_LENGTH;
use sss_unpack::file_entry::FileEntry;

extern crate clap;
use clap::{App, Arg};

// Given the first file, determines the size (in sectors) of the header.
// The header may be one or more sectors, depending on how many files there are,
// and will always be padded out to an even sector boundary.
fn get_header_length(data: &[u8]) -> Result<usize, String> {
    // The index of the first file comes at the sector right after
    // the header ends.
    // Since this is 0-indexed, we can return it directly to get the
    // header length in sectors.
    match FileEntry::parse_file_listing(&data[0..24]) {
        Some(f) => return Ok(f.start as usize),
        None => return Err(String::from("Unable to parse header!")),
    }
}

fn parse_files_from_header(data: &[u8]) -> Vec<FileEntry> {
    let header_size = get_header_length(&data[0..24]);
    // The header is padded out to an even sector boundary, so some of the 24-byte
    // chunks we slice here are going to be 0-byte and will parse to None
    return data[0..SECTOR_LENGTH * header_size.unwrap()]
        .chunks(24)
        .filter_map(|header| FileEntry::parse_file_listing(header))
        .collect();
}

fn validate_input_path(input_path: &Path) -> Result<(), String> {
    if !input_path.exists() {
        return Err(format!(
            "The specified input file ({}) does not exist!",
            input_path.to_string_lossy()
        ));
    } else if !input_path.is_file() {
        return Err(format!(
            "The specified input file ({}) is not a file!",
            input_path.to_string_lossy()
        ));
    }

    return Ok(());
}

fn validate_target_path(target_path: &Path) -> Result<(), String> {
    if !target_path.is_dir() {
        return Err(format!(
            "The specified target directory ({}) is invalid!",
            target_path.to_string_lossy()
        ));
    }

    return Ok(());
}

fn write_file(data: &[u8], file: &FileEntry, target_path: &Path) -> Result<(), String> {
    let file_path = &target_path.join(&file.name);
    match File::create(file_path) {
        Ok(mut f) => {
            let starting_position = file.start as usize * SECTOR_LENGTH;
            match f.write_all(&data[starting_position..starting_position + file.length as usize]) {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!(
                        "Error encountered while writing file {}: {}",
                        &file.name, e
                    ))
                }
            }
        }
        Err(e) => {
            return Err(format!(
                "Unable to create a file at path {}: {}",
                file_path.to_string_lossy(),
                e
            ))
        }
    }

    return Ok(());
}

fn do_stuff(input: String, target: String) -> Result<(), String> {
    let input_path = Path::new(&input);
    let target_path = Path::new(&target);

    validate_input_path(input_path)?;
    validate_target_path(target_path)?;

    let input_file = File::open(&input_path).unwrap();
    let mut buf_reader = BufReader::new(input_file);
    let mut data: Vec<u8> = Vec::new();
    match buf_reader.read_to_end(&mut data) {
        Ok(_) => {}
        Err(e) => return Err(format!("Unable to read file {}: {}", input, e)),
    }

    let files = parse_files_from_header(&data);

    let unpacked_path = target_path.join(input_path.file_name().unwrap());
    if !unpacked_path.is_dir() {
        match fs::create_dir(&unpacked_path) {
            Ok(_) => {}
            Err(e) => return Err(format!("Unable to create directory to unpack in: {}", e)),
        }
    }

    for file in files {
        write_file(&data, &file, &unpacked_path)?;
    }

    return Ok(());
}

fn main() {
    let matches = App::new("sss_unpack")
        .version("0.3.0")
        .author("Misty De Meo")
        .about("Unpack Lunar: Silver Star Story Complete data files")
        .arg(
            Arg::with_name("input")
                .help("The file to unpack")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("target")
                .help("Directory into which unpacked files should be saved")
                .index(2),
        )
        .get_matches();
    let input = matches.value_of("input").unwrap().to_string();
    let target = matches.value_of("target").unwrap_or(".").to_string();

    exit(match do_stuff(input, target) {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    });
}
