use derive_tokio_io::{AsyncRead, AsyncWrite};

#[derive(AsyncRead, AsyncWrite)]
struct Generic<IO> {
    _stream: IO,
}

#[derive(AsyncRead, AsyncWrite)]
struct GenericTuple<IO>(IO);

#[derive(AsyncRead, AsyncWrite)]
struct GenericMultiField<A, B, IO> {
    #[async_read]
    #[async_write]
    _stream: IO,
    _field1: A,
    _field2: B,
}

#[derive(AsyncRead, AsyncWrite)]
struct GenericMultiFieldTuple<A, B, IO>(
    #[async_read]
    #[async_write]
    IO,
    A,
    B,
);

fn main() {}
