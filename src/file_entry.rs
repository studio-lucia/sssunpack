use std::io;
use std::path::PathBuf;

use crate::consts::SECTOR_LENGTH;
use crate::utils::{uint16_from_bytes, uint16_to_bytes, uint32_from_bytes, uint32_to_bytes};

pub struct FileEntry {
    pub name: String,
    pub start: u16,
    pub end: u16,
    pub length: u32,
}

impl FileEntry {
    pub fn parse_file_listing(data: &[u8]) -> Option<FileEntry> {
        // Filename can't begin with nul bytes, so this indicates
        // dummy data.
        if data[0] == 0x0 {
            return None;
        }

        // Filename is up to 12 bytes, padded out with nul bytes we want to strip
        let filename = data[0..12]
            .iter()
            .cloned()
            .take_while(|i| *i != 0)
            .collect();

        return Some(FileEntry {
            name: String::from_utf8(filename).unwrap(),
            start: uint16_from_bytes([data[14], data[15]]),
            end: uint16_from_bytes([data[18], data[19]]),
            length: uint32_from_bytes([data[20], data[21], data[22], data[23]]),
        });
    }

    pub fn serialize(&self) -> Vec<u8> {
        // Filename is exactly 12 bytes, with nul byte padding
        // 12 bytes is 8.3 with the period separator,
        // but smaller filenames are obviously possible.
        let mut data = Vec::from(self.name.as_bytes());
        data.resize(12, 0);

        data.append(&mut uint16_to_bytes(0));
        data.append(&mut uint16_to_bytes(self.start));
        data.append(&mut uint16_to_bytes(0));
        data.append(&mut uint16_to_bytes(self.end));
        data.append(&mut uint32_to_bytes(self.length));

        assert_eq!(24, data.len());

        return data;
    }
}

fn fold_vecs<T>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.extend(b);
    return a;
}

pub struct FileList {
    pub files: Vec<FileEntry>,
}

impl FileList {
    pub fn build(files: &Vec<PathBuf>) -> Result<FileList, io::Error> {
        // Check to see if the directory size is larger than one sector;
        // more than ~85 files requires a multi-sector header.
        // The index of the first file is the next sector boundary following the
        // header's end.
        let header_size = files.len() * 24;
        let mut index = (header_size / SECTOR_LENGTH) + 1;

        let mut file_entries = vec![];
        for file in files {
            let file_length = file.metadata()?.len();
            let end_boundary = ((index as u64 + file_length) / 2048) + 1;

            file_entries.push(FileEntry {
                name: String::from(file.file_name().unwrap().to_str().unwrap()),
                start: index as u16,
                end: end_boundary as u16,
                length: file_length as u32,
            });

            index += end_boundary as usize;
        }

        return Ok(FileList {
            files: file_entries,
        });
    }

    pub fn serialize(&self) -> io::Result<Vec<u8>> {
        let mut serialized = self
            .files
            .iter()
            .map(|file| file.serialize())
            .fold(vec![], fold_vecs);

        // Pad out with 00s to reach an even sector boundary.
        let padded_size = ((serialized.len() / SECTOR_LENGTH) * SECTOR_LENGTH) + SECTOR_LENGTH;
        serialized.resize(padded_size, 0);

        return Ok(serialized);
    }
}
