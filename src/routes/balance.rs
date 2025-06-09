use actix_web::{HttpResponse, Responder, get, web};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use tokio::task;

///  GET /balance/{pubkey}
/// Fetches the SOL balance for a given public key from Solana devnet
#[get("/balance/{pubkey}")]
async fn balance_handler(pubkey_str: web::Path<String>) -> impl Responder {
    let pubkey_str = pubkey_str.into_inner();

    //  Run Solana RPC call in a blocking-safe thread

    let result = task::spawn_blocking(move || {
        let pubkey =
            Pubkey::from_str(&pubkey_str).map_err(|_| "Invalid public key format".to_string())?; // Convert the string into a Pubkey; fail if invalid

        //  Connect to Solana devnet
        let client = RpcClient::new("https://api.devnet.solana.com");
        //  Attempt to fetch balance

        client
            .get_balance(&pubkey)
            .map_err(|e| format!("Failed to fetch balance: {}", e))
    })
    .await;
    // Return appropriate HTTP response

    match result {
        Ok(Ok(balance)) => HttpResponse::Ok().body(format!("Balance: {} lamports", balance)),
        Ok(Err(msg)) => HttpResponse::BadRequest().body(msg),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

/// ✅ Unit test for /balance with invalid input
/// Expects 400 Bad Request for malformed public key
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn test_balance_handler_with_invalid_pubkey() {
        // Build a test app with the balance route
        let app = test::init_service(App::new().service(balance_handler)).await;

        // Call with invalid pubkey
        let req = test::TestRequest::get()
            .uri("/balance/invalid_pubkey_123")
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Check response is 400 BadRequest
        assert_eq!(resp.status(), 400);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Invalid public key format");
    }

    ///  Integration test for /balance with a real devnet pubkey
    /// Ensures the endpoint correctly fetches real balances (network-dependent)
    #[actix_web::test]
    #[ignore] // Prevents from running unless explicitly requested
    async fn test_balance_handler_real_pubkey() {
        let app = test::init_service(App::new().service(balance_handler)).await;

        let req = test::TestRequest::get()
            .uri("/balance/4Nd1mT9C89PxyA2rF5KJbfYyjpTHZf1dbpVqv9d5Dx6z")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }
}
