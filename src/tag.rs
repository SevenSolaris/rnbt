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

#[test]
fn scratch() {
    
}