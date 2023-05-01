use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

//comments.

#[derive(Deserialize)]
pub struct AddUser {
    username: String,
    password: String,
    public_key: String,
}
pub async fn add_user(data: web::Json<AddUser>) -> HttpResponse {
    // Interact with OrbitDB here
    // Return a JSON response
    HttpResponse::Ok().json("User added")
}
// pub struct Web2Login {
//     username: String,
//     password: String,
// }

// pub struct Web3Login {
//     signing_hash: String,
// }

// pub enum Login {
//     Web2Login(Web2Login),
//     Web3Login(Web3Login),
// }
// 






// fn confirm_transaction() ->  {

// }
//Web2 Logins
//Web3 Logins