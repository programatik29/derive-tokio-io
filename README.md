# derive-tokio-io

Derive [`AsyncRead`] and [`AsyncWrite`].

# Usage

If the struct has only one field, [`AsyncRead`] and [`AsyncWrite`] are derived
for that field.

```rust
use derive_tokio_io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

#[derive(AsyncRead, AsyncWrite)]
struct Wrapper {
    stream: TcpStream,
}
```

Generics are supported.

```rust
use derive_tokio_io::{AsyncRead, AsyncWrite};

#[derive(AsyncRead, AsyncWrite)]
struct Wrapper<IO> {
    stream: IO,
}
```

If the struct has multiple fields, `#[async_read]` and `#[async_write]`
must be used once for any field.

```rust
use derive_tokio_io::{AsyncRead, AsyncWrite};

#[derive(AsyncRead, AsyncWrite)]
struct Wrapper<R, W> {
    #[async_read]
    reader: R,
    #[async_write]
    writer: W,
}
```

Everything works the same way for tuple structs.

```rust
use derive_tokio_io::{AsyncRead, AsyncWrite};

#[derive(AsyncRead, AsyncWrite)]
struct Wrapper<R, W>(
    #[async_read]
    R,
    #[async_write]
    W,
);
```

# License

This project is licensed under the [MIT license](./LICENSE).

[`AsyncRead`]: https://docs.rs/tokio/1/tokio/io/trait.AsyncRead.html
[`AsyncWrite`]: https://docs.rs/tokio/1/tokio/io/trait.AsyncWrite.html
