use super::*;

#[derive(Debug)]
pub struct Handshake<T: VarIntLen + ToString> {
    data_len: u8,
    packet_id: u8,
    protocol_version: &'static [u8],
    hostname: Box<T>,
    port: u16,
    next_state: u8,
    new_len: u8,
    new_packet_id: u8,
}

impl<T: VarIntLen + ToString> Handshake<T> {
    pub fn new(hostname: T, port: u16) -> Handshake<T> {
        const SLICE: &[u8] = &[0xf6, 0x05];
        // const SLICE: &[u8] = &[0xff, 0xff, 0xff, 0xff, 0x0f];
        let data_len: u8 = hostname.len() + SLICE.len() as u8 + 5;
        // println!("hostname len: {}", hostname.len());
        Handshake {
            data_len,
            packet_id: 0,
            // protocol_version: &[0xff, 0xff, 0xff, 0xff, 0x0f],
            protocol_version: SLICE,
            hostname: Box::new(hostname),
            port,
            next_state: 1,
            new_len: 1,
            new_packet_id: 0,
        }
    }

    // fn with_protocol_version(mut self, protocol_version: i32) -> Handshake {
    //     let encoding_size: &'static Vec<u8> = protocol_version.encode_var_vec();
    //     self.protocol_version = encoding_size.as_slice();
    //     self
    // }

    pub fn to_byte_vec(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.data_len);
        bytes.push(self.packet_id);
        bytes.extend_from_slice(&self.protocol_version);
        bytes.extend_from_slice(&self.hostname.varint_len());
        bytes.extend_from_slice(&self.hostname.to_string().as_bytes());
        bytes.extend_from_slice(&self.port.to_be_bytes());
        bytes.push(self.next_state);
        bytes.push(self.new_len);
        bytes.push(self.new_packet_id);
        bytes
    }
}

impl<T: VarIntLen + ToString> ToString for Handshake<T> {
    fn to_string(&self) -> String {
        format!("{}:{}", self.hostname.to_string(), self.port)
    }
}
