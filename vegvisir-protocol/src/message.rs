use defmt::Format;
use serde::{Deserialize, Serialize};

/// Possible messages
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Format, Default)]
pub enum Message {
	/// Heartbeat message
	#[default]
	Heartbeat
}