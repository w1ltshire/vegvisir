use crc::{Crc, CRC_32_ISCSI};
use defmt::Format;
use heapless::Vec;
use serde::{Deserialize, Serialize};
use crate::error::ProtocolResult;
use crate::message::Message;

/// Structure representing a single packet
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Format, Default)]
#[non_exhaustive]
pub struct Packet {
	/// LoRa address for P2P communications, can be ignored if using another transport
	pub lora_address: u16,
	/// Payload of the packet
	pub payload: Message
}

impl Packet {
	/// Create a packet from a slice (serialized data by [`postcard`])
	pub fn from_slice(bytes: &[u8]) -> ProtocolResult<Packet> {
		let crc = Crc::<u32>::new(&CRC_32_ISCSI);
		Ok(postcard::from_bytes_crc32(bytes, crc.digest())?)
	}

	/// Create a packet from a [`Vec`] (serialized data by [`postcard`])
	pub fn from_vec(bytes: Vec<u8, 255>) -> ProtocolResult<Packet> {
		let crc = Crc::<u32>::new(&CRC_32_ISCSI);
		Ok(postcard::from_bytes_crc32(&bytes, crc.digest())?)
	}

	/// Serialize this packet with [`postcard`]
	pub fn serialize(&self) -> ProtocolResult<Vec<u8, 255>> {
		let crc = Crc::<u32>::new(&CRC_32_ISCSI);
		// 4 bytes = 32 bits of checksum
		let size = postcard::serialize_with_flavor(self, postcard::ser_flavors::Size::default())? + 4;
		let mut buf: Vec<u8, 255> = Vec::new();
		buf.resize(size, 0)?;
		postcard::to_slice_crc32(self, &mut buf, crc.digest())?;
		Ok(buf)
	}
}

#[cfg(test)]
mod tests {
	extern crate std;
	//extern crate alloc;

	use crate::packet::Packet;
	use std::println;
	use crate::message::Message;

	const EXAMPLE_PACKET: Packet = Packet {
		lora_address: 0xF9F9,
		payload: Message::Heartbeat
	};

	const BYTES: [u8; 8] = [0xF9, 0xF3, 0x3, 0x0, 0x19, 0xA8, 0x4C, 0xA7];

	#[test]
	fn serialize() {
		let ser = EXAMPLE_PACKET.serialize().unwrap();
		println!("{ser:X?}");
		assert_eq!(*ser, [0xF9, 0xF3, 0x3, 0x0, 0x19, 0xA8, 0x4C, 0xA7])
	}

	#[test]
	fn from_bytes() {
		let packet = Packet::from_slice(&BYTES).unwrap();
		println!("{packet:X?}");
		assert_eq!(packet, EXAMPLE_PACKET);
	}
}