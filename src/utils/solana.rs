use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

/// * `Keypair` - usable key object for signing transactions
pub fn load_keypair(path: &str) -> Keypair {
    let mut file = File::open(path).expect("Unable to open keypair file");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read keypair file");

    let bytes: Vec<u8> = serde_json::from_str(&data).expect("Invalid JSON");
    Keypair::from_bytes(&bytes).expect("Invalid keypair")
}

/// Loads KEYPAIR_PATH from .env and returns a Keypair
pub fn load_keypair_from_env() -> Keypair {
    dotenv::dotenv().ok(); // Load .env file if present
    let path = env::var("KEYPAIR_PATH").expect("KEYPAIR_PATH must be set in .env");
    load_keypair(&path)
}

/// # Arguments:
/// * `client` - an instance of `RpcClient` connected to the devnet/mainnet
/// * `sender` - the `Keypair` of the wallet sending SOL
/// * `to` - recipient’s public key (base58 encoded string)
/// * `amount` - number of lamports to send (1 SOL = 1_000_000_000 lamports)
///
/// # Returns:
/// * `Signature` - the Solana transaction signature if successful
pub fn transfer_sol(
    client: &RpcClient,
    sender: &Keypair,
    to: &str,
    amount: u64,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let to_pubkey = Pubkey::from_str(to)?;
    let ix = system_instruction::transfer(&sender.pubkey(), &to_pubkey, amount);
    let recent_blockhash = client.get_latest_blockhash()?;

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&sender.pubkey()),
        &[sender],
        recent_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx)?;
    Ok(sig)
}
