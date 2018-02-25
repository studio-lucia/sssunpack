use std::io::Cursor;

extern crate byteorder;
use self::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub fn uint16_from_bytes(bytes : [u8; 2]) -> u16 {
    return Cursor::new(bytes)
        .read_u16::<BigEndian>()
        .unwrap();
}

pub fn uint32_from_bytes(bytes : [u8; 4]) -> u32 {
    return Cursor::new(bytes)
        .read_u32::<BigEndian>()
        .unwrap();
}

pub fn uint16_to_bytes(int: u16) -> Vec<u8> {
    let mut result = vec![];
    result.write_u16::<BigEndian>(int).unwrap();

    return result;
}

pub fn uint32_to_bytes(int: u32) -> Vec<u8> {
    let mut result = vec![];
    result.write_u32::<BigEndian>(int).unwrap();

    return result;
}
