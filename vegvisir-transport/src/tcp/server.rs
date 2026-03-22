use alloc::string::String;
use tokio::net::TcpListener;
use crate::{Listener, Transport};
use crate::tcp::client::TCPClient;

/// TCP server
pub struct TCPServer {
	listener: TcpListener
}

impl TCPServer {
	/// Create a new instance of [`TCPServer`]
	pub async fn new(host: impl Into<String>) -> TCPServer {
		let listener = TcpListener::bind(host.into()).await.unwrap();
		TCPServer { listener }
	}
}

#[allow(refining_impl_trait)] // for TCPClient in return
impl Listener for TCPServer {
	type Error = tokio::io::Error;

	async fn accept(&self) -> Result<TCPClient, Self::Error> {
		let (stream, _) = self.listener.accept().await?;
		let transport = TCPClient { stream };
		Ok(transport)
	}

	async fn close(self) -> Result<(), Self::Error> {
		drop(self.listener);
		Ok(())
	}
}