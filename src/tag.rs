#![allow(dead_code)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List,
    Compound,
    IntArray,
    LongArray,
}

pub enum ListTag {
    Byte(Vec<i8>),
    Short(Vec<i16>),
}

#[test]
fn scratch() {
    use std::mem::size_of;
    println!("Size: {}", size_of::<ListTag>());
}