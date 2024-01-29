use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use actix::clock::{interval, Interval};
use actix::spawn;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::info;
use tokio::sync::watch;

mod socket;
use socket::WebsocketConnection;

async fn ws_connection(request: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("Client opened websocket connection");
    let state = request
        .app_data::<web::Data<State>>()
        .expect("no state data?")
        .clone();
    let socket = WebsocketConnection::new(state.into_inner().as_ref().clone());
    ws::start(socket, &request, stream)
}

#[derive(Clone)]
struct State {
    tx: Arc<watch::Sender<Option<Vec<u8>>>>,
    rx: watch::Receiver<Option<Vec<u8>>>,
}

impl State {
    pub fn new() -> Self {
        let (tx, rx) = watch::channel(None);
        Self {
            tx: Arc::new(tx),
            rx,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("server=info"));

    // Start image server actor.
    let state = State::new();

    // Start update task.
    spawn(send_updates(state.clone()));

    // Start server.
    info!("Starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/ws", web::get().to(ws_connection))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn send_updates(state: State) {
    let mut interval = interval(Duration::from_millis(50));
    let tx = state.tx.clone();

    let paths = fs::read_dir("./images").expect("images directory missing");

    let mut paths: Vec<PathBuf> = paths
        .into_iter()
        .filter_map(|p| p.ok().map(|p| p.path()))
        .collect();
    paths.sort();

    let images: Vec<(&PathBuf, Vec<u8>)> = paths
        .iter()
        .map(|p| {
            let bytes = fs::read(p).expect("fail to load image from file");
            (p, bytes)
        })
        .collect();

    loop {
        for (_, bytes) in &images {
            tx.send(Some(bytes.clone()))
                .expect("fail to send on watch channel");
            interval.tick().await;
        }
    }
}
