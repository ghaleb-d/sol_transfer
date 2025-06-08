use actix_web::{App, HttpServer, get, Responder};
mod routes;
mod models;
use routes::transfer::transfer_handler;

#[actix_web::main] // Macro that starts the async runtime using Tokio
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello) // Registers our route `/`
        .service(transfer_handler)   // POST /transfer ✅ Now registered
    })
    .bind(("127.0.0.1", 8080))? // Binds to localhost:8080
    .run()
    .await
}

#[get("/")] //Attribute macro (#[macro]) to ask Actix to route Get /
async fn hello() -> impl Responder { // type implementing Responder trait
    "Hello, Rust + Solana!"
}


