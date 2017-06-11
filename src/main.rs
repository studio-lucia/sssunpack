use std::env;
use std::fs::File;
use std::iter::Iterator;
// use std::io;
use std::path::Path;
use std::process::exit;

extern crate clap;
use clap::{Arg, App};

fn do_stuff(input : String, target : String) -> Result<(), String> {
    let input_path = Path::new(&input);
    let target_path = Path::new(&target);

    if !input_path.exists() {
        return Err(format!("The specified input file ({}) does not exist!", input));
    } else if !input_path.is_file() {
        return Err(format!("The specified input file ({}) is not a file!", input));
    }

    if !target_path.parent().unwrap().is_dir() {
        return Err(format!("The specified target directory ({}) is invalid!", target));
    }

    let mut input_file = File::open(&input_path);

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
                              .required(true)
                              .index(2))
                          .get_matches();
    let input = matches.value_of("input").unwrap().to_string();
    let target = matches.value_of("target").unwrap().to_string();

    exit(match do_stuff(input, target) {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        },
    });
}
