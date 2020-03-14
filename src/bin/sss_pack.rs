use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;

use clap::{Arg, App};

use sss_unpack::consts::SECTOR_LENGTH;
use sss_unpack::file_entry::FileList;

fn write_file(file: PathBuf, writer: &mut BufWriter<File>) -> io::Result<u64> {
    let file_length = file.metadata()?.len();
    let padding_size = SECTOR_LENGTH - (file_length as usize % SECTOR_LENGTH);

    let input_file = File::open(&file)?;
    let mut buf_reader = BufReader::new(input_file);

    let mut bytes_to_read = file_length as usize;
    while bytes_to_read > 0 {
        // We define the buffer in here so anything past the number of
        // bytes written is 0 rather than leftover bytes from the last
        // read.
        let mut buf = vec![0; SECTOR_LENGTH];
        let read_bytes = buf_reader.read(&mut buf)?;
        writer.write(&buf)?;
        bytes_to_read -= read_bytes;
    }

    // No off-by-one errors here please
    let bytes_written = file_length + padding_size as u64;
    debug_assert!((bytes_written % 2048) == 0);
    return Ok(bytes_written);
}

fn process(input_files: Vec<PathBuf>, target: PathBuf) -> io::Result<()> {
    let target_file = File::create(target)?;
    let mut writer = BufWriter::new(target_file);

    let file_list = FileList::build(&input_files)?;
    writer.write(&file_list.serialize()?)?;

    for file in input_files {
        write_file(file, &mut writer)?;
    }

    return Ok(());
}

fn main() {
    let matches = App::new("sss_pack")
                          .version("0.3.0")
                          .author("Misty De Meo")
                          .about("Pack Lunar: Silver Star Story Complete data files")
                          .arg(Arg::with_name("target")
                              .help("The packed filename")
                              .required(true)
                              .index(1))
                          .arg(Arg::with_name("input")
                              .help("File(s) to pack")
                              .required(true)
                              .multiple(true))
                          .get_matches();
    let target = matches.value_of("target").unwrap();
    let target_path = PathBuf::from(&target);

    let mut input_files = matches.values_of("input").unwrap().map(|path| PathBuf::from(path)).collect::<Vec<PathBuf>>();
    // Thanks APFS
    input_files.sort();
    if input_files.iter().any(|path| !path.exists()) {
        println!("One or more input files couldn't be found!");
        exit(1);
    }

    exit(match process(input_files, target_path) {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        },
    });
}
