//  Import core Actix Web framework components
use actix_web::{App, HttpServer, Responder, get};

//  Declare project modules (located in /models, /routes, /utils)
mod models;
mod routes;
mod utils;

//  Import the handlers defined in your route modules
use actix_cors::Cors;
use routes::balance::balance_handler;
use routes::transfer::transfer_handler;

#[actix_web::main] //  Macro that initializes an async runtime using `tokio`
// Entry point of your Actix web server
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // Loads environment variables from `.env` into the app (e.g., wallet path)

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // allow all origins for dev
            .service(hello) // ➕ GET / -> returns greeting
            .service(transfer_handler) // ➕ POST /transfer -> sends SOL
            .service(balance_handler) // ➕ GET /balance/{pubkey} -> fetch balance
    })
    .bind(("127.0.0.1", 8080))? // Bind server to localhost:8080
    .run() // Launch the server
    .await // Wait for shutdown
}

#[get("/")] // Route macro for handling GET requests to "/"
async fn hello() -> impl Responder {
    //  Simple handler that returns a static string response
    "Hello, Rust + Solana!"
}
