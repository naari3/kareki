// referenced https://docs.rs/crate/bufstream/0.1.4
// because idk how to overwrite part of implementation
use std::error;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};

use std::net::TcpStream;

use aes::Aes128;
use cfb8::stream_cipher::{NewStreamCipher, StreamCipher};
use cfb8::Cfb8;

pub type AesCfb8 = Cfb8<Aes128>;

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub struct McStream {
    buffer: Vec<u8>,
    inner: TcpStream,
    encryptor: Option<AesCfb8>,
    decryptor: Option<AesCfb8>,
}

impl McStream {
    /// Creates a new buffered stream with explicitly listed capacities for the
    /// reader/writer buffer.
    pub fn with_capacities(writer_cap: usize, inner: TcpStream) -> McStream {
        let mut buffer = vec![];
        // unsafe { buffer.set_len(writer_cap) };
        McStream {
            inner: inner,
            encryptor: None,
            decryptor: None,
            buffer: buffer,
        }
    }

    /// Creates a new buffered stream with the default reader/writer buffer
    /// capacities.
    pub fn new(inner: TcpStream) -> McStream {
        McStream::with_capacities(DEFAULT_BUF_SIZE, inner)
    }

    pub fn set_encryptor(&mut self, key: &[u8]) {
        self.encryptor = AesCfb8::new_var(&key, &key).ok();
    }

    pub fn set_decryptor(&mut self, key: &[u8]) {
        self.decryptor = AesCfb8::new_var(&key, &key).ok();
    }
}

impl Read for McStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.inner.read(buf)?;
        if let Some(decryptor) = self.decryptor.as_mut() {
            decryptor.decrypt(buf);
        }
        Ok(res)
    }
}

impl Write for McStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        if let Some(encryptor) = self.encryptor.as_mut() {
            encryptor.encrypt(&mut self.buffer);
        }
        self.inner.write_all(&self.buffer)?;
        self.buffer = vec![];
        Ok(())
    }
}
