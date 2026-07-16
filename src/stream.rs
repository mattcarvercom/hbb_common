use crate::{tcp, ResultType};
use sodiumoxide::crypto::secretbox::Key;
use std::net::SocketAddr;
use tokio::net::TcpStream;

/// The LAN-only transport is always a direct TCP stream.
pub struct Stream(tcp::FramedStream);

impl Stream {
    #[inline]
    pub fn set_send_timeout(&mut self, ms: u64) {
        self.0.set_send_timeout(ms);
    }

    #[inline]
    pub fn set_raw(&mut self) {
        self.0.set_raw();
    }

    #[inline]
    pub async fn send_bytes(&mut self, bytes: bytes::Bytes) -> ResultType<()> {
        self.0.send_bytes(bytes).await
    }

    #[inline]
    pub async fn send_raw(&mut self, bytes: Vec<u8>) -> ResultType<()> {
        self.0.send_raw(bytes).await
    }

    #[inline]
    pub fn set_key(&mut self, key: Key) {
        self.0.set_key(key);
    }

    #[inline]
    pub fn is_secured(&self) -> bool {
        self.0.is_secured()
    }

    #[inline]
    pub async fn next_timeout(
        &mut self,
        timeout: u64,
    ) -> Option<Result<bytes::BytesMut, std::io::Error>> {
        self.0.next_timeout(timeout).await
    }

    #[inline]
    pub async fn send(&mut self, msg: &impl protobuf::Message) -> ResultType<()> {
        self.0.send(msg).await
    }

    #[inline]
    pub async fn next(&mut self) -> Option<Result<bytes::BytesMut, std::io::Error>> {
        self.0.next().await
    }

    #[inline]
    pub fn local_addr(&self) -> SocketAddr {
        self.0.local_addr()
    }

    #[inline]
    pub fn from(stream: TcpStream, stream_addr: SocketAddr) -> Self {
        Self(tcp::FramedStream::from(stream, stream_addr))
    }

    #[inline]
    pub fn from_framed(stream: tcp::FramedStream) -> Self {
        Self(stream)
    }
}
