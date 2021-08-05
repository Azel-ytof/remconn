//! # SshPacket
//!
//! TODO
//!

use rand::Rng;

pub struct SshPacket {
    packet_length: u32,
    padding_length: u8,
    payload: String,
    random_padding: Vec<u8>,
    // mac: String,
}

impl SshPacket {
    fn packet_length(&self) {
        // size of padding_length + payload + random_padding
        todo!()
    }

    fn payload(&self) -> &[u8] {
        let expected_length: usize = self.packet_length as usize - self.padding_length as usize - 1;
        let payload_bytes = self.payload.as_bytes();

        if payload_bytes.len() != expected_length {
            todo!()
        }

        payload_bytes
    }

    fn generate_random_padding(&self) -> Vec<u8> {
        let mut a = vec![0; self.padding_length as usize];

        let mut rng = rand::thread_rng();
        for b in a.iter_mut() {
            *b = rng.gen();
        }

        a
    }
}

impl Default for SshPacket {
    fn default() -> Self {
        Self {
            packet_length: 0,
            padding_length: 8,
            payload: String::from(""),
            random_padding: Vec::new(),
            // mac: ,
        }
    }
}
