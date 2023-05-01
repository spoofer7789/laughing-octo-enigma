
use actix::prelude::*;
use actix_web::{get,web, App, HttpRequest, HttpResponse, HttpServer, post};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use super::state::{AppState,MyWs};

//change to UseRequests later
pub enum UserData {
    Logins(Logins),
    CreateAccount(CreateAccount),
    Web3(Web3),
}
#[derive(Deserialize, Serialize, Clone)]
struct Logins {
    username: String,
    password: String,
}

struct CreateAccount {
    email: String,
    username: String,
    password: String,
}

struct Web3 {
    publickey: String,
    signature: String,
}
#[post("/create_account")]
fn create_account(data: web::Json<CreateAccount>) -> HttpResponse {
    // Handle the account creation logic here
    // listen in for usernames
    //listen in for emails 
    // listen in for passwords
    if () // the username and email aren't taken, create account,
    //else return email or username are taken
}

// Regular login
#[get("/login")]
async fn login(data: web::Json<Logins>) -> HttpResponse {
    // Handle the login logic here
}

// Web3 login
#[get("/web3_login")]
async fn web3_login(data: web::Json<Web3>) -> HttpResponse {
    // Handle the Web3 login logic here
    //If the users signs in using metamask, take in there publickey and 
}


pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(MyWs { app_data: data }, &req, stream).map_err(|err| {
        eprintln!("WebSocket error: {:?}", err);
        err
    })
}