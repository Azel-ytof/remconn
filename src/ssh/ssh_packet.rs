use rand::Rng;

/// # SshPacket
///
/// A SSH packet corresponding to RFC description
///
pub struct SshPacket {
    packet_length: u32,
    padding_length: u8,
    payload: String,
    random_padding: Vec<u8>,
    // mac: String,
}

impl SshPacket {
    pub fn new(payload: String) -> Self {
        let padding_length = 8;
        let random_padding = SshPacket::generate_random_padding(padding_length);
        let payload_length = payload.len() as u32;
        // 1 is for padding_length byte
        let packet_length = 1 + payload_length + padding_length as u32;

        Self {
            packet_length,
            padding_length,
            payload,
            random_padding,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut final_packet: Vec<u8> = Vec::new();

        final_packet.extend_from_slice(&self.packet_length.to_be_bytes());
        final_packet.push(self.padding_length);
        final_packet.extend_from_slice(self.payload.as_bytes());
        final_packet.extend(self.random_padding.clone());
        // final_packet.push(self.mac);

        final_packet
    }

    fn generate_random_padding(padding_length: u8) -> Vec<u8> {
        let mut a = vec![0; padding_length as usize];

        let mut rng = rand::thread_rng();
        for b in a.iter_mut() {
            *b = rng.gen();
        }

        a
    }
}
