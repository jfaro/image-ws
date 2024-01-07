use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use actix::{Actor, Addr};
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod server;
mod session;

use server::ImageServer;
use session::WebsocketImageSession;

async fn image_route(
    request: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ImageServer>>,
) -> Result<HttpResponse, Error> {
    let session = WebsocketImageSession::new(0, server.get_ref().clone());
    let response = ws::start(session, &request, stream);
    println!("Response: {:?}", response);
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize application state.
    let app_state = Arc::new(AtomicUsize::new(0));

    // Start image server actor.
    let server = ImageServer::new(app_state.clone()).start();

    log::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(image_route))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
