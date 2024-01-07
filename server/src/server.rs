use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use actix::{Actor, Context, Handler, Recipient};
use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New session created.
#[derive(actix::Message)]
#[rtype(result = "usize")]
pub struct Connect {
    pub address: Recipient<Message>,
}

/// Session is disconnected.
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Manage client sessions.
#[derive(Debug)]
pub struct ImageServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rng: ThreadRng,
}

impl ImageServer {
    pub fn new(_: Arc<AtomicUsize>) -> Self {
        Self {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl ImageServer {
    fn send_message(&self, message: &str) {
        for (id, recipient) in &self.sessions {
            println!("Sending to {}: {}", id, message);
            recipient.do_send(Message(message.to_owned()))
        }
    }
}

/// Make actor from `ImageServer`.
impl Actor for ImageServer {
    type Context = Context<Self>;
}

/// Handler for connect message.
impl Handler<Connect> for ImageServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen::<usize>();
        println!("Client joined: {}", id);
        self.sessions.insert(id, msg.address);
        id
    }
}

/// Handler for disconnect message.
impl Handler<Disconnect> for ImageServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        println!("Client disconnected: {}", msg.id);
        self.sessions.remove(&msg.id);
    }
}
