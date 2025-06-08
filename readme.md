# 📜 Rust Backend for Solana Token Transfers

This is a simple backend written in **Rust** using the **Actix-Web** framework. It provides two endpoints:

* `GET /` – A test route to ensure the server is running
* `POST /transfer` – Accepts a JSON request to initiate a transfer between two Solana addresses

This project is beginner-friendly and documents every concept to help new Rust developers understand how to structure a backend.

---

## 📁 Project Structure

```
.
├── Cargo.toml
├── src
│   ├── main.rs               # Entry point of the Actix web server
│   ├── routes
│   │   ├── mod.rs
│   │   └── transfer.rs       # POST /transfer endpoint handler
│   └── models
│       ├── mod.rs
│       └── transfer.rs       # Defines TransferRequest struct for JSON deserialization
```

---

## ✅ Features

* Written 100% in Rust
* Powered by [Actix-Web](https://actix.rs/)
* JSON deserialization via [`serde`](https://serde.rs/)
* Modular and idiomatic Rust folder structure
* Ready to integrate with the [Solana Rust SDK](https://docs.rs/solana-sdk/)

---

## 🚀 Getting Started

### 1. Clone the Repo

```bash
git clone https://github.com/your-username/rust-solana-transfer-backend.git
cd rust-solana-transfer-backend
```

### 2. Run the Server

```bash
cargo run
```

The server will run on `http://127.0.0.1:8080`.

---

## 🔍 Testing Endpoints

### `GET /`

Open your browser or run:

```bash
curl http://127.0.0.1:8080/
```

You should see:

```text
Hello, Rust + Solana!
```

---

### `POST /transfer`

You can test using `curl`, Postman, or Insomnia.

**Example `curl` Command:**

```bash
curl -X POST http://127.0.0.1:8080/transfer \
  -H "Content-Type: application/json" \
  -d '{"from": "wallet1", "to": "wallet2", "amount": 1000}'
```

Expected response:

```text
Transfer request received.
```

Server log will show:

```text
Request to transfer 1000 lamports from wallet1 to wallet2
```

---

## 🧠 How It Works

### `main.rs`

* Starts the Actix web server.
* Registers routes like `/` and `/transfer`.

### `models/transfer.rs`

Defines the data structure used to deserialize incoming JSON for the transfer request.

```rust
#[derive(Deserialize)]
pub struct TransferRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
}
```

### `routes/transfer.rs`

Handles the logic for the `POST /transfer` endpoint. Currently, it just logs and acknowledges the request.

---

## 🔮 Coming Next: Solana Integration

The next steps are:

* Connect to the Solana network (Devnet or Localnet)
* Use the `solana-client` and `solana-sdk` crates
* Create and sign transactions
* Send tokens between addresses

We are keeping this modular and progressive — a great learning base for Rust and Solana developers.

---

## 📚 Learning Goals

This repo is built with the goal of learning and teaching:

* How to build REST APIs in idiomatic Rust
* How to work with asynchronous Rust using Actix
* How to integrate Rust with Web3 technologies like Solana

---

## 🧑‍💻 Author

Built by a Rust + Web3 enthusiast on a journey to master the Solana ecosystem.
Follow the repo to stay updated as we add transaction signing and Solana logic.

---

## 💠 Stack

* Language: **Rust**
* Framework: **Actix-Web**
* Web3: **Solana (coming soon)**
* JSON: **Serde**

---

## 🪙 Example Use Case

Once the Solana logic is added, this API will be able to:

* Trigger wallet-to-wallet token transfers on-chain
* Serve as a backend service for dApps, dashboards, or bots

---

## 📄 License

MIT
