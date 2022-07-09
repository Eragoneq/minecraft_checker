use super::*;

#[derive(Debug)]
pub struct Ip {
    i1: u8,
    i2: u8,
    i3: u8,
    i4: u8,
}

impl Ip {
    pub fn new(i1: u8, i2: u8, i3: u8, i4: u8) -> Self {
        Self { i1, i2, i3, i4 }
    }
}

impl VarIntLen for Ip {
    fn len(&self) -> u8 {
        self.to_string().len() as u8
    }

    fn varint_len(&self) -> Vec<u8> {
        let vec = self.len().encode_var_vec();
        // println!("{:?}", vec);
        vec
    }
}

impl ToString for Ip {
    fn to_string(&self) -> String {
        format!("{}.{}.{}.{}", self.i1, self.i2, self.i3, self.i4)
    }
}
