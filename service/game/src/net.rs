use std::net::ToSocketAddrs;

pub mod udp;
pub mod ws;

pub trait NetworkServer {
    fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, anyhow::Error>
    where
        Self: Sized;

    /// Read avaiable message to buffer
    fn read(&mut self, buffer: &mut Vec<ClientMessage>);

    /// Send a message to UserId
    ///
    /// Return Some(true) on success
    ///        None if this server do not contains this net id
    fn write(&mut self, id: &UserNetId, buffer: &[u8]) -> Option<bool>;

    /// Flush pending write message
    fn flush(&mut self);
}

pub type UserId = u32;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserNetId(u32);

impl UserNetId {
    fn next() -> Self {
        use std::sync::atomic::{AtomicU32, Ordering};

        static ID_GEN: AtomicU32 = AtomicU32::new(0);

        Self(ID_GEN.fetch_add(1, Ordering::SeqCst))
    }
}

#[derive(Debug)]
pub struct ClientMessage {
    net_id: UserNetId,
    data: Vec<u8>,
}

impl ClientMessage {
    pub fn net_id(&self) -> UserNetId {
        self.net_id
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
