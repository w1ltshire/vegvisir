use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::Transport;

/// TCP server
pub struct TCPClient {
	pub(crate) stream: TcpStream,
}

impl Transport for TCPClient {
	type Error = tokio::io::Error;

	async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
		self.stream.write_all(data).await?;
		Ok(())
	}

	async fn recv(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
		self.stream.read_exact(buf).await?;
		Ok(())
	}
}