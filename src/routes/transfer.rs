use actix_web::{post, web, Responder, HttpResponse};
use crate::models::transfer::TransferRequest;


#[post("/transfer")]
pub async fn transfer_handler(req: web::Json<TransferRequest>) -> impl Responder {
    let data = req.into_inner();
    println!("Request to transfer {} lamports from {} to {}", data.amount, data.from, data.to);
    HttpResponse::Ok().body("Transfer request received.")
}
