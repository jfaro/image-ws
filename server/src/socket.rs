use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, Handler, Running, SpawnHandle, StreamHandler};
use actix_web_actors::ws;
use log::info;
use uuid::Uuid;

use crate::State;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebsocketConnection {
    /// Unique session identifier.
    pub id: Uuid,
    /// Last client heartbeat.
    pub heartbeat: Instant,
    /// Application state.
    pub state: State,
    /// Receiver task handle.
    pub handle: Option<SpawnHandle>,
}

impl WebsocketConnection {
    pub fn new(state: State) -> Self {
        Self {
            id: Uuid::new_v4(),
            heartbeat: Instant::now(),
            state,
            handle: None,
        }
    }
}

impl Actor for WebsocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection starting for {:?}", self.id);
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                info!("Websocket client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"")
        });

        let mut rx = self.state.rx.clone();
        self.handle = Some(ctx.add_stream(async_stream::stream! {
            while rx.changed().await.is_ok() {
                let value = rx.borrow().to_string();
                log::info!("new stream value: {:?}", value);
                yield value
            };
        }));
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::prelude::Running {
        info!("WebSocket connection stopping for {:?}", self.id);
        Running::Stop
    }
}

impl StreamHandler<String> for WebsocketConnection {
    fn handle(&mut self, msg: String, ctx: &mut Self::Context) {
        log::info!("handling stream value: {msg:?}");
        ctx.text(msg);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketConnection {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(message) => match message {
                ws::Message::Ping(msg) => {
                    info!("Received ping!");
                    self.heartbeat = Instant::now();
                    ctx.pong(&msg)
                }
                ws::Message::Pong(_) => {
                    info!("Received pong!");
                    self.heartbeat = Instant::now();
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
