use std::io::Cursor;

extern crate byteorder;
use self::byteorder::{BigEndian, ReadBytesExt};

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