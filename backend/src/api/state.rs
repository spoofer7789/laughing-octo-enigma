use super::routes::*;
use actix::prelude::*;
use actix_web::{get,web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
pub struct AppState {
    user_data: Mutex<Vec<UserData>>,
}
pub struct MyWs {
    app_data: web::Data<AppState>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(Deserialize, Serialize, Debug)]
struct PostData {
    file: Option<String>,
    text: Option<String>,
    public_key: String,
    is_escrowed: (bool, String),
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let post_data: PostData = serde_json::from_str(&text).unwrap();

                // Process post data
                // ...

                // Send a response back to the client
                ctx.text("Post received");
            }
            _ => (),
        }
    }
}
