use super::*;

#[derive(Debug)]
pub struct Hostname {
    name: String,
}

impl Hostname {
    pub fn new(name: String) -> Self { Self { name } }
}

impl VarIntLen for Hostname {
    fn len(&self) -> u8 {
        self.name.len() as u8
    }

    fn varint_len(&self) -> Vec<u8> {
        let vec = self.len().encode_var_vec();
        // println!("{:?}", vec);
        vec
    }
}

impl ToString for Hostname {
    fn to_string(&self) -> String {
        self.name.to_owned()
    }
}