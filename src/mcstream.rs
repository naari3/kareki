
// referenced https://docs.rs/crate/bufstream/0.1.4
// because idk how to overwrite part of implementation
use std::fmt;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::error;

use std::net::TcpStream;

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

/// Wraps a Stream and buffers input and output to and from it.
///
/// It can be excessively inefficient to work directly with a `Read+Write`. For
/// example, every call to `read` or `write` on `TcpStream` results in a system
/// call. A `BufStream` keeps in memory buffers of data, making large,
/// infrequent calls to `read` and `write` on the underlying `Read+Write`.
///
/// The output buffer will be written out when this stream is dropped.
#[derive(Debug)]
pub struct McStream {
    inner: BufReader<InternalBufWriter<TcpStream>>
}

/// An error returned by `into_inner` which combines an error that
/// happened while writing out the buffer, and the buffered writer object
/// which may be used to recover from the condition.
#[derive(Debug)]
pub struct IntoInnerError<W>(W, io::Error);

impl<W> IntoInnerError<W> {
    /// Returns the error which caused the call to `into_inner()` to fail.
    ///
    /// This error was returned when attempting to write the internal buffer.
    pub fn error(&self) -> &io::Error { &self.1 }
    /// Returns the buffered writer instance which generated the error.
    ///
    /// The returned object can be used for error recovery, such as
    /// re-inspecting the buffer.
    pub fn into_inner(self) -> W { self.0 }
}

impl<W> From<IntoInnerError<W>> for io::Error {
    fn from(iie: IntoInnerError<W>) -> io::Error { iie.1 }
}

impl<W: fmt::Debug> error::Error for IntoInnerError<W> {
    fn description(&self) -> &str {
        error::Error::description(self.error())
    }
}

impl<W> fmt::Display for IntoInnerError<W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error().fmt(f)
    }
}

struct InternalBufWriter<W: Write>(Option<BufWriter<W>>);

impl<W: Write> InternalBufWriter<W> {
    fn get_ref(&self) -> &BufWriter<W> {
        self.0.as_ref().unwrap()
    }

    fn get_mut(&mut self) -> &mut BufWriter<W> {
        self.0.as_mut().unwrap()
    }
}

impl<W: Read + Write> Read for InternalBufWriter<W> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.get_mut().get_mut().read(buf)
    }
}

impl<W: Write + fmt::Debug> fmt::Debug for InternalBufWriter<W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get_ref().fmt(f)
    }
}

impl McStream {
    /// Creates a new buffered stream with explicitly listed capacities for the
    /// reader/writer buffer.
    pub fn with_capacities(reader_cap: usize, writer_cap: usize, inner: TcpStream)
                           -> McStream {
        let writer = BufWriter::with_capacity(writer_cap, inner);
        let internal_writer = InternalBufWriter(Some(writer));
        let reader = BufReader::with_capacity(reader_cap, internal_writer);
        McStream { inner: reader }
    }

    /// Creates a new buffered stream with the default reader/writer buffer
    /// capacities.
    pub fn new(inner: TcpStream) -> McStream {
        McStream::with_capacities(DEFAULT_BUF_SIZE, DEFAULT_BUF_SIZE, inner)
    }

    /// Gets a reference to the underlying stream.
    pub fn get_ref(&self) -> &TcpStream {
        self.inner.get_ref().get_ref().get_ref()
    }

    /// Gets a mutable reference to the underlying stream.
    ///
    /// # Warning
    ///
    /// It is inadvisable to read directly from or write directly to the
    /// underlying stream.
    pub fn get_mut(&mut self) -> &mut TcpStream {
        self.inner.get_mut().get_mut().get_mut()
    }

    /// Unwraps this `BufStream`, returning the underlying stream.
    ///
    /// The internal write buffer is written out before returning the stream.
    /// Any leftover data in the read buffer is lost.
    pub fn into_inner(mut self) -> Result<TcpStream, IntoInnerError<McStream>> {
        let e = {
            let InternalBufWriter(ref mut w) = *self.inner.get_mut();
            let (e, w2) = match w.take().unwrap().into_inner() {
                Ok(s) => return Ok(s),
                Err(err) => {
                    (io::Error::new(err.error().kind(), err.error().to_string()),
                     err.into_inner())
                }
            };
            *w = Some(w2);
            e
        };
        Err(IntoInnerError(self, e))
    }
}

impl BufRead for McStream {
    fn fill_buf(&mut self) -> io::Result<&[u8]> { self.inner.fill_buf() }
    fn consume(&mut self, amt: usize) { self.inner.consume(amt) }
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_until(byte, buf)
    }
    fn read_line(&mut self, string: &mut String) -> io::Result<usize> {
        self.inner.read_line(string)
    }
}

impl Read for McStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for McStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.get_mut().0.as_mut().unwrap().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.get_mut().0.as_mut().unwrap().flush()
    }
}
