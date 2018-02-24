use utils::{uint16_from_bytes, uint32_from_bytes};

pub struct FileEntry {
    pub name: String,
    pub start: u16,
    // This field is contained in the header, so we're still storing it
    // even though currently nothing in this script uses it.
    #[allow(dead_code)]
    pub end: u16,
    pub length: u32,
}

impl FileEntry {
    pub fn parse_file_listing(data : &[u8]) -> Option<FileEntry> {
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
}
