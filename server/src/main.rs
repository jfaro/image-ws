use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

struct Websocket;

impl Actor for Websocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Websocket {
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

async fn index(request: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let response = ws::start(Websocket {}, &request, stream);
    println!("Response: {:?}", response);
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
