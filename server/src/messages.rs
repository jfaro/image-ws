use actix::{Message, Recipient};
use uuid::Uuid;

/// WebsocketConnection responds to this to send inner content to clients.
#[derive(Message)]
#[rtype(result = "()")]
pub struct WebsocketMessage {
    pub content: String,
}

/// WebsocketConnection sends this to the server to connect.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: Uuid,
    pub address: Recipient<WebsocketMessage>,
}

/// WebsocketConnection sends this to the server to disconnect.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}
