# 🦀 Solana Transfer Backend (Rust + Actix Web)

This is a **Rust-based backend server** built with **Actix Web** that connects to the **Solana Devnet** to:

- ✅ Load a wallet from a `.json` file
- ✅ Check balance for any public key
- ✅ Transfer SOL from the backend wallet to any recipient

It is ideal for:
- Testnet/devnet faucets
- Internal automation
- Server-controlled payments

---

## ⚙️ Features

- **RESTful API** with `/balance/{pubkey}` and `/transfer`
- Built on **Actix Web** for high performance
- Uses `solana_client` SDK to communicate with the blockchain
- ✅ Includes error handling and basic unit tests
- ✅ Balance check before sending funds

---

## 🛠️ Prerequisites

- Rust (1.70+)
- Solana CLI installed (for wallet generation)
- `.env` file containing the path to your wallet:

KEYPAIR_PATH=wallet.json


---

## 🚀 Running the Server

```bash
# 1. Install dependencies
cargo build

# 2. Start the server
cargo run
The server will run on:
http://127.0.0.1:8080

📡 Available Endpoints

🔍 GET /balance/{pubkey}
Returns the balance (in lamports) for the given Solana public key.

curl http://127.0.0.1:8080/balance/YourPublicKeyHere

💸 POST /transfer
Transfers SOL from the backend wallet to a specified recipient.

Example Request:

curl -X POST http://127.0.0.1:8080/transfer \
  -H "Content-Type: application/json" \
  -d '{"to": "RecipientPubkeyHere", "amount": 1000000}'