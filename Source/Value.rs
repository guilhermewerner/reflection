use crate::Object;

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Char(char),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Float32(f32),
    Float64(f64),
    String(String),
    Object(Object),
}
