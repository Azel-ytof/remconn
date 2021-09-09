use crate::errors::SshError;
use rand::Rng;

/// A SSH packet corresponding to [RFC4253 documentation]
///
/// After creating a `SshPacket` by giving the message to send, you can convert it [`into_bytes`] to send it through a buffer
///
/// [RFC4253 documentation]: https://datatracker.ietf.org/doc/html/rfc4253
/// [`into_bytes`]: SshPacket::into_bytes
pub struct SshPacket {
    packet_length: u32,
    padding_length: u8,
    payload: String,
    random_padding: Vec<u8>,
    // mac: String,
}

impl SshPacket {
    /// Create an instance by giving the message to send
    pub fn new(payload: String) -> Self {
        let padding_length = rand::thread_rng().gen_range(8..255);
        let random_padding = SshPacket::generate_random_padding(padding_length);
        // 1 is for padding_length byte
        let packet_length = 1 + payload.len() as u32 + padding_length as u32;

        // todo : packet_length must be encrypted

        Self {
            packet_length,
            padding_length,
            payload,
            random_padding,
        }
    }

    /// Convert and consume the current instance to a vector of u8
    pub fn into_bytes(self) -> Result<Vec<u8>, SshError> {
        let mut final_packet: Vec<u8> = Vec::new();

        final_packet.extend_from_slice(&self.packet_length.to_be_bytes());
        final_packet.push(self.padding_length);
        final_packet.extend_from_slice(self.payload.as_bytes());
        final_packet.extend(self.random_padding);

        if final_packet.len() < 16 {
            return Err(SshError::new(String::from(
                "The minimum size of the packet is less than 16 bytes",
            )));
        }

        if final_packet.len() % 8 != 0 {
            return Err(SshError::new(String::from(
                "The size of the packet must be a multiple of 8",
            )));
        }

        // final_packet.push(self.mac);

        if final_packet.len() > 35000 {
            return Err(SshError::new(String::from(
                "The maximum size of the packet is more than 3500 bytes",
            )));
        }

        Ok(final_packet)
    }

    /// Generate the random padding based on length specified while instanciating the struct
    fn generate_random_padding(padding_length: u8) -> Vec<u8> {
        let mut a = vec![0; padding_length as usize];

        let mut rng = rand::thread_rng();
        for b in a.iter_mut() {
            *b = rng.gen();
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_padding() {
        let length: usize = 8;
        let random_padding = SshPacket::generate_random_padding(length as u8);

        assert_eq!(
            length,
            random_padding.len(),
            "Verifying random padding length"
        );
    }
}
