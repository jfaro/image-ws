use actix::{Actor, Addr};
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::info;

mod messages;
mod server;
mod session;

use server::ImageServer;
use session::WebsocketConnection;

async fn ws_connection(
    request: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ImageServer>>,
) -> Result<HttpResponse, Error> {
    info!("Client opened websocket connection");
    let session = WebsocketConnection::new(server.get_ref().clone());
    ws::start(session, &request, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("server=info"));

    // Start image server actor.
    let server = ImageServer::default().start();

    info!("Starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(ws_connection))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
