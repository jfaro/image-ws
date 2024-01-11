use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Running, StreamHandler};
use actix_web_actors::ws;
use log::info;
use uuid::Uuid;

use crate::messages::{Disconnect, WebsocketMessage};
use crate::server::ImageServer;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebsocketConnection {
    /// Unique session identifier.
    pub id: Uuid,
    /// Image server address.
    pub address: Addr<ImageServer>,
    /// Last client heartbeat.
    pub last_heartbeat: Instant,
}

impl WebsocketConnection {
    pub fn new(address: Addr<ImageServer>) -> Self {
        Self {
            id: Uuid::new_v4(),
            address,
            last_heartbeat: Instant::now(),
        }
    }

    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |conn, ctx| {
            if Instant::now().duration_since(conn.last_heartbeat) > CLIENT_TIMEOUT {
                info!("Websocket client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"PING")
        });
    }
}

impl Actor for WebsocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection starting for {:?}", self.id);
        self.heartbeat(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::prelude::Running {
        info!("WebSocket connection stopping for {:?}", self.id);
        self.address.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketConnection {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(message) => match message {
                ws::Message::Ping(msg) => {
                    info!("Received ping!");
                    self.last_heartbeat = Instant::now();
                    ctx.pong(&msg)
                }
                ws::Message::Pong(_) => {
                    info!("Received pong!");
                    self.last_heartbeat = Instant::now();
                }
                ws::Message::Text(text) => {
                    info!("Received text: {text}");
                    ctx.text(text);
                }
                ws::Message::Binary(binary) => {
                    info!("Received binary: {binary:?}");
                    ctx.binary(binary);
                }
                ws::Message::Close(reason) => {
                    log::info!("Received close: {reason:?}");
                    ctx.close(reason);
                    ctx.stop();
                }
                ws::Message::Continuation(msg) => {
                    info!("Received continuation: {msg:?}");
                    ctx.stop();
                }
                ws::Message::Nop => (),
            },
            Err(e) => panic!("{}", e), // TODO: Handle errors.
        }
    }
}

impl Handler<WebsocketMessage> for WebsocketConnection {
    type Result = ();

    fn handle(&mut self, msg: WebsocketMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.content)
    }
}
