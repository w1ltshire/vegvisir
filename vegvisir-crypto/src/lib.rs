#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![warn(missing_docs)]
#![no_std]
extern crate alloc;

#[cfg(test)]
mod tests {
	extern crate std;

	use alloc::string::String;
	use alloc::vec;
	use alloc::vec::Vec;
	use vegvisir_transport::Transport;
	use std::println;

	pub struct MockTransport {
		pub receiver: Vec<u8>,
		pub sender: Vec<u8>,
	}

	impl Transport for MockTransport {
		type Error = std::io::Error;

		async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
			self.receiver = data.to_vec();
			Ok(())
		}

		async fn recv(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
			if self.sender.len() > buf.len() {
				return Err(std::io::Error::new(
					std::io::ErrorKind::InvalidInput,
					"buffer too small for incoming data",
				));
			}
			buf[..self.sender.len()].copy_from_slice(&self.sender);
			Ok(())
		}
	}

	#[tokio::test]
	async fn session() {
		let mut initiator = snow::Builder::new("Noise_NN_25519_ChaChaPoly_BLAKE2s".parse().unwrap())
			.build_initiator().unwrap();

		let mut out_buf = [0u8; 255];
		let len = initiator.write_message(&[], &mut out_buf).unwrap();
		let handshake_msg = &out_buf[..len];

		let mut transport = MockTransport {
			receiver: Vec::new(),
			sender: Vec::new(),
		};
		transport.send(handshake_msg).await.unwrap();

		let mut responder = snow::Builder::new("Noise_NN_25519_ChaChaPoly_BLAKE2s".parse().unwrap())
			.build_responder().unwrap();
		let mut resp_buf = [0u8; 255];
		responder.read_message(handshake_msg, &mut resp_buf).unwrap();
		let resp_len = responder.write_message(&[], &mut resp_buf).unwrap();
		let response_msg = &resp_buf[..resp_len];

		transport.sender = response_msg.to_vec();

		let mut inbound = [0u8; 255];
		transport.recv(&mut inbound).await.unwrap();
		let inbound_len = response_msg.len();
		let inbound_slice = &inbound[..inbound_len];

		initiator.read_message(inbound_slice, &mut out_buf).unwrap();
		let mut initiator = initiator.into_transport_mode().unwrap();

		let payload = b"mmrrrooooow";
		let mut encrypted = vec![0u8; payload.len() + 16];
		initiator.write_message(payload, &mut encrypted).unwrap();
		let mut responder = responder.into_transport_mode().unwrap();

		println!("{:X?}", encrypted);

		let mut plaintext = vec![0u8; encrypted.len() + 16];
		let plain_len = responder.read_message(&*encrypted, &mut plaintext).unwrap();

		let plaintext = &plaintext[..plain_len];
		println!("{}", String::from_utf8_lossy(plaintext));
	}
}