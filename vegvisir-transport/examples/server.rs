use vegvisir_transport::Transport;
use vegvisir_transport::Listener;
use vegvisir_transport::tcp::server;

#[tokio::main]
async fn main() {
	let server = server::TCPServer::new("127.0.0.1:1234").await;
	server.accept().await.unwrap().send(b"meow mrooow mew meoooooww\n").await.unwrap();
}
