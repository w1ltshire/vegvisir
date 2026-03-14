use crc::{Crc, CRC_32_ISCSI};
use defmt::Format;
use heapless::Vec;
use serde::{Deserialize, Serialize};

/// Structure representing a single packet
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Format)]
pub struct Packet {
	/// LoRa address for P2P communications, can be ignored if using another transport
	pub lora_address: u16,
}

impl Packet {
	/// Serialize this packet with [`postcard`]
	pub fn serialize(&self) -> Vec<u8, 255> {
		let crc = Crc::<u32>::new(&CRC_32_ISCSI);
		// 4 bytes = 32 bits of checksum
		let size = postcard::serialize_with_flavor(self, postcard::ser_flavors::Size::default()).unwrap() + 4;
		let mut buf: Vec<u8, 255> = Vec::new();
		buf.resize(size, 0).unwrap();
		postcard::to_slice_crc32(self, &mut buf, crc.digest()).unwrap();
		buf
	}
}

#[cfg(test)]
mod tests {
	extern crate std;
	//extern crate alloc;

	use crate::packet::Packet;
	use std::println;

	const EXAMPLE_PACKET: Packet = Packet {
		lora_address: 0xF9F9,
	};

	#[test]
	fn serialize() {
		let ser = EXAMPLE_PACKET.serialize();
		println!("{ser:X?}");
	}
}