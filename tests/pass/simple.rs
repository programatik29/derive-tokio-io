use derive_tokio_io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

#[derive(AsyncRead, AsyncWrite)]
struct Simple {
    _stream: TcpStream,
}

#[derive(AsyncRead, AsyncWrite)]
struct SimpleTuple(TcpStream);

fn main() {}
