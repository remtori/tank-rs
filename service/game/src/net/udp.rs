use std::{
    collections::{HashMap, HashSet},
    io::ErrorKind,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
};

use anyhow::Context;

use super::{ClientMessage, NetworkServer, UserNetId};

pub struct UdpNetServer {
    socket: UdpSocket,
    clients: HashMap<UserNetId, SocketAddr>,
    bad_clients: HashSet<UserNetId>,
    addr_to_net_id: HashMap<SocketAddr, UserNetId>,
}

impl UdpNetServer {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, anyhow::Error> {
        let socket = UdpSocket::bind(addr).context("Bind new UdpSocket")?;
        socket.set_nonblocking(true).context("Set UdpSocket to non blocking")?;

        Ok(Self {
            socket,
            clients: HashMap::new(),
            bad_clients: HashSet::new(),
            addr_to_net_id: HashMap::new(),
        })
    }

    /// Map SocketAddress to UserNetId
    ///
    /// Create new mapping if non found
    fn addr_to_net_id(&mut self, addr: SocketAddr) -> UserNetId {
        use std::collections::hash_map::Entry;
        match self.addr_to_net_id.entry(addr) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let id = UserNetId::next();
                entry.insert(id);
                self.clients.insert(id, addr);
                id
            }
        }
    }

    fn remove_bad_clients(&mut self) {
        for net_id in self.bad_clients.drain() {
            if let Some(addr) = self.clients.remove(&net_id) {
                self.addr_to_net_id.remove(&addr);
            }
        }
    }
}

impl NetworkServer for UdpNetServer {
    fn read(&mut self, message_buffer: &mut Vec<ClientMessage>) {
        let buffer = &mut [0; 4096];

        loop {
            match self.socket.recv_from(buffer) {
                Ok((n_read, addr)) => {
                    assert!(n_read > 0, "UdpSocket read 0 bytes");

                    message_buffer.push(ClientMessage {
                        net_id: self.addr_to_net_id(addr),
                        data: buffer[..n_read].to_vec(),
                    });
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => {
                    // out of data, skip
                    break;
                }
                Err(err) => {
                    panic!("Udp Read Error: {:?}", err);
                }
            }
        }

        self.remove_bad_clients();
    }

    fn write(&mut self, id: &UserNetId, buffer: &[u8]) -> bool {
        let addr = match self.clients.get(id) {
            Some(addr) => *addr,
            None => return false,
        };

        let mut buffer = buffer;
        loop {
            match self.socket.send_to(buffer, addr) {
                Ok(n_write) => {
                    if n_write == buffer.len() {
                        return true;
                    }

                    buffer = &buffer[n_write..];
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => {
                    // hmmm
                    log::info!("Udp Write Would Block!!");
                }
                Err(err) => {
                    log::error!("Udp Write Error: {:?}", err);
                    self.bad_clients.insert(*id);
                    break;
                }
            }
        }

        self.remove_bad_clients();
        false
    }

    fn flush(&mut self) {
        self.remove_bad_clients();
    }
}
