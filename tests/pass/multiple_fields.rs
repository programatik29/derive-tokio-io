use derive_tokio_io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

#[derive(AsyncRead, AsyncWrite)]
struct MultiFields {
    #[async_read]
    #[async_write]
    _stream: TcpStream,
    _field1: (),
    _field2: (),
}

#[derive(AsyncRead, AsyncWrite)]
struct MultiFieldsTuple(
    #[async_read]
    #[async_write]
    TcpStream,
    (),
    (),
);

fn main() {}
