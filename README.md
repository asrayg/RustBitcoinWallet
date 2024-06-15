# Rust Bitcoin Wallet Application

This project is a simple Bitcoin wallet application built with a Rust-based backend and a Yew-based frontend. The application allows users to derive addresses, generate QR codes, view transaction history, generate new addresses, and make transactions.

## Project Structure

The project is divided into two main parts:
1. **Backend**: Handles Bitcoin wallet functionalities such as deriving addresses, generating QR codes, viewing transaction history, generating new addresses, and making transactions.
2. **Frontend**: A web interface built using Yew that interacts with the backend to provide a user-friendly experience.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Trunk](https://trunkrs.dev/#install)

### Backend Setup

1. **Navigate to the backend directory:**

   ```bash
   cd bitcoin_wallet_backend
   ```

2. **Add dependencies:**

   ```bash
   cargo add warp serde serde_json tokio bitcoin bip32 chrono
   ```

3. **Run the backend:**

   ```bash
   cargo run
   ```

### Frontend Setup

1. **Navigate to the frontend directory:**

   ```bash
   cd bitcoin_wallet_frontend
   ```

2. **Add dependencies:**

   ```bash
   cargo add yew wasm-bindgen-futures reqwest --features "wasm" serde --features "derive" serde_json stylist
   ```

3. **Build and serve the frontend:**

   ```bash
   trunk serve
   ```

### Current Functionality

- **Derive Address**: Derives a Bitcoin address from a given seed phrase and derivation path.
- **Generate QR Code**: Generates a QR code for the derived address.
- **View Transaction History**: Displays the transaction history.
- **Generate New Address**: Generates a new Bitcoin address.
- **Make Transaction**: Allows making a transaction to a specified recipient address with a specified amount.

### Known Issues

- **QR Code Generation**: The implementation for generating QR codes is currently mocked and needs to be implemented properly.
- **Transaction Creation and Broadcasting**: The transaction creation and broadcasting logic is currently mocked and needs to be implemented with actual Bitcoin transaction logic.
- **Error Handling**: Proper error handling needs to be added for various operations.
- **Styling**: The frontend styling is basic and can be improved for a better user experience.

### Contribution

This project is still a work in progress, and any contributions or suggestions are welcome. If you encounter any issues or have improvements, feel free to open an issue or submit a pull request.

---

Thank you for checking out this project. Your contributions and feedback are highly appreciated!