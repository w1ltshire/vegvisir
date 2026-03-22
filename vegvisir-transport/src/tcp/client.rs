use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::Transport;

/// TCP server
pub struct TCPClient {
	pub(crate) stream: TcpStream,
}

impl TCPClient {
	/// Create a new [`TCPClient`] instance
	pub fn new(stream: TcpStream) -> Self {
		Self { stream }
	}
}

impl Transport for TCPClient {
	type Error = tokio::io::Error;

	async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
		self.stream.write_all(data).await?;
		Ok(())
	}

	async fn recv(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
		self.stream.read(buf).await?;
		Ok(())
	}
}