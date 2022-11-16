//! Derive [`AsyncRead`] and [`AsyncWrite`].
//!
//! # Usage
//!
//! If the struct has only one field, [`AsyncRead`] and [`AsyncWrite`] are derived
//! for that field.
//!
//! ```rust
//! use derive_tokio_io::{AsyncRead, AsyncWrite};
//! use tokio::net::TcpStream;
//! 
//! #[derive(AsyncRead, AsyncWrite)]
//! struct Wrapper {
//!     stream: TcpStream,
//! }
//! ```
//!
//! Generics are supported.
//!
//! ```rust
//! use derive_tokio_io::{AsyncRead, AsyncWrite};
//! 
//! #[derive(AsyncRead, AsyncWrite)]
//! struct Wrapper<IO> {
//!     stream: IO,
//! }
//! ```
//!
//! If the struct has multiple fields, `#[async_read]` and `#[async_write]`
//! must be used once for any field.
//!
//! ```rust
//! use derive_tokio_io::{AsyncRead, AsyncWrite};
//! 
//! #[derive(AsyncRead, AsyncWrite)]
//! struct Wrapper<R, W> {
//!     #[async_read]
//!     reader: R,
//!     #[async_write]
//!     writer: W,
//! }
//! ```
//!
//! Everything works the same way for tuple structs.
//!
//! ```rust
//! use derive_tokio_io::{AsyncRead, AsyncWrite};
//! 
//! #[derive(AsyncRead, AsyncWrite)]
//! struct Wrapper<R, W>(
//!     #[async_read]
//!     R,
//!     #[async_write]
//!     W,
//! );
//! ```
//!
//! [`AsyncRead`]: https://docs.rs/tokio/1/tokio/io/trait.AsyncRead.html
//! [`AsyncWrite`]: https://docs.rs/tokio/1/tokio/io/trait.AsyncWrite.html

use async_read::expand_async_read;
use async_write::expand_async_write;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod async_read;
mod async_write;
mod parse;

/// Derive an implementation of [`AsyncRead`].
///
/// # Example
///
/// ```rust
/// # use derive_tokio_io::AsyncRead;
/// #[derive(AsyncRead)]
/// struct ReadWrapper<R> {
///     reader: R,
/// }
/// ```
///
/// ```rust
/// # use derive_tokio_io::AsyncRead;
/// #[derive(AsyncRead)]
/// struct ReadWrapper<R> {
///     #[async_read]
///     reader: R,
///     some_field: String,
/// }
/// ```
///
/// [`AsyncRead`]: https://docs.rs/tokio/1/tokio/io/trait.AsyncRead.html
#[proc_macro_derive(AsyncRead, attributes(async_read))]
pub fn derive_async_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_async_read(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derive an implementation of [`AsyncWrite`].
///
/// # Example
///
/// ```rust
/// # use derive_tokio_io::AsyncWrite;
/// #[derive(AsyncWrite)]
/// struct WriteWrapper<W> {
///     writer: W,
/// }
/// ```
///
/// ```rust
/// # use derive_tokio_io::AsyncWrite;
/// #[derive(AsyncWrite)]
/// struct WriteWrapper<W> {
///     #[async_write]
///     writer: W,
///     some_field: String,
/// }
/// ```
///
/// [`AsyncWrite`]: https://docs.rs/tokio/1/tokio/io/trait.AsyncWrite.html
#[proc_macro_derive(AsyncWrite, attributes(async_write))]
pub fn derive_async_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_async_write(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.pass("tests/pass/*.rs");
    }
}
