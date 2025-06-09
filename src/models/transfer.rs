use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransferRequest {
    pub to: String,
    pub amount: u64,
}
