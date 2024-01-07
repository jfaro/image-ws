use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, Addr, AsyncContext, StreamHandler};
use actix_web_actors::ws;

use crate::server::{self, ImageServer};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebsocketImageSession {
    /// Unique session identifier.
    pub id: usize,
    /// Image server address.
    pub address: Addr<ImageServer>,
    /// Last client heartbeat.
    pub last_heartbeat: Instant,
}

impl WebsocketImageSession {
    pub fn new(id: usize, address: Addr<ImageServer>) -> Self {
        Self {
            id,
            address,
            last_heartbeat: Instant::now(),
        }
    }

    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |session, ctx| {
            if Instant::now().duration_since(session.last_heartbeat) > CLIENT_TIMEOUT {
                println!("Websocket client heartbeat failed, disconnecting!");
                session
                    .address
                    .do_send(server::Disconnect { id: session.id });
                ctx.stop();

                // Don't send ping.
                return;
            }

            ctx.ping(b"")
        });
    }
}

impl Actor for WebsocketImageSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketImageSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(message) => match message {
                ws::Message::Ping(msg) => ctx.pong(&msg),
                ws::Message::Text(text) => ctx.text(text),
                ws::Message::Binary(binary) => ctx.binary(binary),
                message => println!("Unhandled message: {message:?}"),
            },
            Err(e) => println!("Error: {e:?}"),
        }
    }
}
