use super::*;

#[derive(Debug)]
pub struct Hostname<'a> {
    name: &'a str,
}

impl Hostname<'_> {
    pub fn new(name: &'static str) -> Self { Self { name } }
}

impl VarIntLen for Hostname<'_> {
    fn len(&self) -> u8 {
        self.name.len() as u8
    }

    fn varint_len(&self) -> Vec<u8> {
        let vec = self.len().encode_var_vec();
        // println!("{:?}", vec);
        vec
    }
}

impl ToString for Hostname<'_> {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}