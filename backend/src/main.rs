use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

enum AppState {
    logins: Mutex<Vec<Logins>>,
    create_account: Mutex<Vec<CreateAccount>>,
    web3: Mutex<Vec<Web3>>
}
#[derive(Deserialize,Serialize, Clone)]
struct Logins {
    username: string,
    password: string,
}

struct CreateAccount {
    email: string,
    username: string,
    password: string,
}
struct Web3 {
    publickey: string,
    signature: string,
}
#[get("/")]
async fn index() -> String {
    "this is a health check route".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data -> web::Data::new(AppState {
        logins: Mutex::new(vec![]) || create_account: Mutex::new(vec![]) || web3: Mutex::new(vec![])
    });
    HttpServer::new(move || {
        App::new().app_data(app_data.clone())
        .service(index
      )
    })
    .bind(("127.0.0.1", 3000))?
    .run().await
}