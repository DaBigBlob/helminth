// QUIC style variable length unsigned integers

pub enum UInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128)
}