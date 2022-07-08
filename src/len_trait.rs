pub trait VarIntLen {
    fn len(&self) -> u8;
    fn varint_len(&self) -> Vec<u8>;
}
