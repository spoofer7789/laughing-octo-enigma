mod api;
mod controllers;
mod utils;
use actix::prelude::*;
use actix_web::{get,web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use api::routes::ws_route;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use controllers::routes::add_user;
// Define an enum for the different types of data

// Update AppState to have one Vec<UserData> field




#[get("/")]
async fn index() -> String {
    "this is a health check route".to_string()
}
//server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize AppState with one Vec<UserData> field
    let app_data = web::Data::new(AppState {
        user_data: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new().app_data(app_data.clone())
        .service(index)
        // .route("/create_account", web::post().to(create_account))
        // .route("/login", web::post().to(login))
        // .route("/web3_login", web::post().to(web3_login))
        // .route("/make_post", web::post().to(make_post))
        .route("/ws/", web::get().to(ws_route))
        .route("/add_user", web::post().to(add_user))
    })
    .bind(("127.0.0.1", 3000))?
    .run().await
}