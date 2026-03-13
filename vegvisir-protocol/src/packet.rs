use defmt::Format;
use serde::{Deserialize, Serialize};

/// Structure representing a single packet
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Format)]
pub struct Packet {
	/// LoRa address for P2P communications, can be ignored if using another transport
	pub lora_address: u16,
}

#[cfg(test)]
mod tests {
	extern crate std;

	use crate::packet::Packet;
	use crc::{Crc, CRC_32_ISCSI};
	use std::println;

	const EXAMPLE_PACKET: Packet = Packet {
		lora_address: 0xF9F9
	};

	#[test]
	fn serialize() {
		let crc = Crc::<u32>::new(&CRC_32_ISCSI);
		let mut buf = [0u8; 10];
		let ser = postcard::to_slice_crc32(&EXAMPLE_PACKET, &mut buf, crc.digest()).unwrap();
		println!("{ser:X?}");
	}
}