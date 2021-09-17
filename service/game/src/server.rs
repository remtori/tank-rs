use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io,
    net::{TcpListener, TcpStream},
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use tungstenite::{
    handshake::{
        server::{Callback, ErrorResponse, Request, Response},
        MidHandshake,
    },
    protocol::WebSocketConfig,
    HandshakeError, Message, ServerHandshake, WebSocket,
};

use crate::timer::Timer;

const WEBSOCKET_CONFIG: WebSocketConfig = WebSocketConfig {
    max_send_queue: Some(1),
    max_message_size: Some(8 << 20),
    max_frame_size: Some(4 << 20),
    accept_unmasked_frames: false,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Loading,
    Playing,
}

struct WsCallback;

impl Callback for WsCallback {
    fn on_request(self, _request: &Request, response: Response) -> Result<Response, ErrorResponse> {
        Ok(response)
    }
}

type ServerMidHandshake = MidHandshake<ServerHandshake<TcpStream, WsCallback>>;

type WS = WebSocket<TcpStream>;

#[derive(Debug, Clone)]
struct Client {
    id: u32,
    ws: Rc<RefCell<WS>>,
}

impl Client {
    pub fn new(ws: WS) -> Self {
        static ID_GEN: AtomicU32 = AtomicU32::new(0);

        Self {
            id: ID_GEN.fetch_add(1, Ordering::SeqCst),
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

pub struct Server {
    id: String,
    tps: u32,
    state: State,
    listener: TcpListener,
    clients: HashMap<u32, Client>,
    pending_handshake: Vec<ServerMidHandshake>,
}

impl Server {
    pub fn new(id: String, tps: u32, listener: TcpListener) -> Self {
        Self {
            id,
            tps,
            state: State::Idle,
            listener,
            clients: HashMap::new(),
            pending_handshake: Vec::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        let timer = &mut Timer::new(self.tps);

        loop {
            let num_ticks = timer.begin();
            let num_ticks = if num_ticks > self.tps as u32 {
                eprintln!("Can't keep up, skipped {} tick", num_ticks - 1);
                1
            } else {
                num_ticks
            };

            for _ in 0..num_ticks {
                match self.state {
                    State::Idle => self.process_command(),
                    State::Loading | State::Playing => {
                        // Accept new or reconnecting user
                        self.accept_new_incoming();

                        let bad_clients = &mut HashSet::new();

                        self.clients
                            .iter_mut()
                            .for_each(|(id, client)| match client.read_message() {
                                Ok(msg @ Message::Text(_)) | Ok(msg @ Message::Binary(_)) => {
                                    // Simple echo server for testing purpose
                                    match client.write_message(msg) {
                                        Ok(_) => {}
                                        Err(tungstenite::Error::Io(err)) => {
                                            if err.kind() == io::ErrorKind::WouldBlock {
                                                // out of data, skip
                                            } else {
                                                bad_clients.insert(*id);
                                                // eprintln!("IO Write Error: {:?}", err);
                                            }
                                        }
                                        Err(err) => {
                                            eprintln!("Write Error: {:?}", err);
                                        }
                                    }
                                }
                                Ok(_) => {
                                    // ignore other type of message
                                }
                                Err(tungstenite::Error::Io(err)) => {
                                    if err.kind() == io::ErrorKind::WouldBlock {
                                        // out of data, skip
                                    } else {
                                        bad_clients.insert(*id);
                                        // eprintln!("IO Read Error: {:?}", err);
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Read Error: {:?}", err);
                                }
                            });

                        // Clean up, flush pending write
                        let mut pending_flush_clients = self
                            .clients
                            .values()
                            .filter(|client| !bad_clients.contains(&client.id))
                            .collect::<Vec<_>>();

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

                        // remove all bad clients
                        self.clients.retain(|id, _| !bad_clients.contains(id));
                    }
                }
            }

            timer.end();
        }
    }

    /// Process command from master
    fn process_command(&mut self) {
        self.state = State::Loading;
    }

    fn accept_new_incoming(&mut self) {
        let clients = &mut self.clients;

        // Poll previous pending handshake
        self.pending_handshake = self
            .pending_handshake
            .drain(..)
            .filter_map(|hs| match hs.handshake() {
                Ok(ws) => {
                    let client = Client::new(ws);
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
                        let client = Client::new(ws);
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
}
