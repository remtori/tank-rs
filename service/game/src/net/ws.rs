use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    rc::Rc,
};

use anyhow::Context;
use tungstenite::{
    handshake::{
        server::{Callback, ErrorResponse, Request, Response},
        MidHandshake,
    },
    protocol::WebSocketConfig,
    HandshakeError, Message, ServerHandshake, WebSocket,
};

use super::{NetworkServer, UserNetId};
use crate::net::ClientMessage;

const WEBSOCKET_CONFIG: WebSocketConfig = WebSocketConfig {
    max_send_queue: Some(1),
    max_message_size: Some(8 << 20),
    max_frame_size: Some(4 << 20),
    accept_unmasked_frames: false,
};

struct WsCallback;

impl Callback for WsCallback {
    fn on_request(self, _request: &Request, response: Response) -> Result<Response, ErrorResponse> {
        Ok(response)
    }
}

type ServerMidHandshake = MidHandshake<ServerHandshake<TcpStream, WsCallback>>;

type WS = WebSocket<TcpStream>;

#[derive(Debug, Clone)]
struct WsClient {
    id: UserNetId,
    ws: Rc<RefCell<WS>>,
}

impl WsClient {
    pub fn new(ws: WS) -> Self {
        Self {
            id: UserNetId::next(),
            ws: Rc::new(RefCell::new(ws)),
        }
    }

    pub fn read_message(&self) -> Result<Message, tungstenite::Error> {
        self.ws.borrow_mut().read_message()
    }

    pub fn write_message(&self, message: Message) -> Result<(), tungstenite::Error> {
        self.ws.borrow_mut().write_message(message)
    }

    pub fn write_pending(&self) -> Result<bool, tungstenite::Error> {
        match self.ws.borrow_mut().write_pending() {
            Ok(_) => Ok(false),
            Err(tungstenite::Error::Io(err)) if err.kind() == io::ErrorKind::WouldBlock => Ok(false),
            Err(err) => Err(err),
        }
    }
}

pub struct WsNetServer {
    listener: TcpListener,
    clients: HashMap<UserNetId, WsClient>,
    bad_clients: HashSet<UserNetId>,
    pending_handshake: Vec<ServerMidHandshake>,
}

impl WsNetServer {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, anyhow::Error> {
        let listener = TcpListener::bind(addr).context("Bind new TcpListener")?;
        listener
            .set_nonblocking(true)
            .context("Set TcpListener to non blocking")?;

        Ok(Self {
            listener,
            clients: HashMap::new(),
            bad_clients: HashSet::new(),
            pending_handshake: Vec::new(),
        })
    }

    fn accept_new_incoming(&mut self) {
        let clients = &mut self.clients;

        // Poll previous pending handshake
        self.pending_handshake = self
            .pending_handshake
            .drain(..)
            .filter_map(|hs| match hs.handshake() {
                Ok(ws) => {
                    let client = WsClient::new(ws);
                    clients.insert(client.id, client);
                    None
                }
                Err(HandshakeError::Interrupted(handshake)) => Some(handshake),
                Err(err) => {
                    eprintln!("Handshake Error: {:?}", err);
                    None
                }
            })
            .collect();

        // Accept new connection
        for incoming in self.listener.incoming() {
            match incoming {
                Ok(stream) => match tungstenite::accept_hdr_with_config(stream, WsCallback, Some(WEBSOCKET_CONFIG)) {
                    Ok(ws) => {
                        let client = WsClient::new(ws);
                        clients.insert(client.id, client);
                    }
                    Err(HandshakeError::Interrupted(handshake)) => {
                        self.pending_handshake.push(handshake);
                    }
                    Err(err) => {
                        eprintln!("Handshake Error: {:?}", err)
                    }
                },
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                    // Accepted all queued connection
                    break;
                }
                Err(err) => {
                    eprintln!("Accepting Error: {:?}", err)
                }
            }
        }
    }

    fn remove_bad_clients(&mut self) {
        let bad_clients = &mut self.bad_clients;
        self.clients.retain(|id, _| !bad_clients.contains(id));
        self.bad_clients.clear();
    }
}

impl NetworkServer for WsNetServer {
    fn read(&mut self, buffer: &mut Vec<ClientMessage>) {
        self.accept_new_incoming();

        let bad_clients = &mut self.bad_clients;

        self.clients
            .iter()
            .for_each(|(id, client)| match client.read_message() {
                Ok(msg @ Message::Text(_)) | Ok(msg @ Message::Binary(_)) => {
                    buffer.push(ClientMessage {
                        net_id: *id,
                        data: msg.into_data(),
                    });
                }
                Ok(_) => {
                    // ignore other type of message
                }
                Err(tungstenite::Error::Io(err)) => {
                    if err.kind() == io::ErrorKind::WouldBlock {
                        // out of data, skip
                    } else {
                        // eprintln!("IO Read Error: {:?}", err);
                        bad_clients.insert(*id);
                    }
                }
                Err(err) => {
                    eprintln!("Read Error: {:?}", err);
                }
            });
    }

    fn write(&mut self, id: &UserNetId, buffer: &[u8]) -> bool {
        let client = match self.clients.get(id) {
            Some(client) => client,
            None => return false,
        };

        match client.write_message(Message::Binary(buffer.to_vec())) {
            Ok(_) => true,
            Err(tungstenite::Error::Io(err)) => {
                if err.kind() == io::ErrorKind::WouldBlock {
                    // need flush later
                    true
                } else {
                    self.bad_clients.insert(*id);
                    // eprintln!("IO Write Error: {:?}", err);
                    false
                }
            }
            Err(err) => {
                eprintln!("Write Error: {:?}", err);
                false
            }
        }
    }

    fn flush(&mut self) {
        self.remove_bad_clients();

        let mut pending_flush_clients = self.clients.values().collect::<Vec<_>>();
        loop {
            pending_flush_clients = pending_flush_clients
                .into_iter()
                .filter_map(|client| match client.write_pending() {
                    Ok(true) => Some(client),
                    Ok(false) => None,
                    Err(err) => {
                        eprintln!("Write Error: {:?}", err);
                        None
                    }
                })
                .collect();

            if pending_flush_clients.is_empty() {
                break;
            }
        }

        self.remove_bad_clients();
    }
}
