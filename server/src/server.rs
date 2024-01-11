use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};
use log::info;
use uuid::Uuid;

use crate::messages::{Connect, Disconnect, WebsocketMessage};

type Socket = Recipient<WebsocketMessage>;

/// Manage client sessions.
#[derive(Debug)]
pub struct ImageServer {
    /// Map client IDs to the client connection.
    sessions: HashMap<Uuid, Socket>,
}

impl Default for ImageServer {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl ImageServer {
    /// Sends message to all clients.
    fn send_message(&self, message: &str) {
        for (id, recipient) in &self.sessions {
            info!("Sending to {}: {}", id, message);
            recipient.do_send(WebsocketMessage {
                content: message.to_owned(),
            })
        }
    }
}

/// Make actor from `ImageServer`.
impl Actor for ImageServer {
    type Context = Context<Self>;
}

/// Handler for connect message.
impl Handler<Connect> for ImageServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        log::info!("Client joined: {}", msg.id);
        self.sessions.insert(msg.id, msg.address);
        self.send_message(&format!("your id is {}", msg.id));
    }
}

/// Handler for disconnect message.
impl Handler<Disconnect> for ImageServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        info!("Client disconnected: {}", msg.id);
        self.sessions.remove(&msg.id);
    }
}
