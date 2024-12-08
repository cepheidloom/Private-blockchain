# Private Blockchain Implementation
This is a blockchain application built with Tauri, SvelteKit, and Rust. This project demonstrates the integration of a blockchain backend with a modern frontend framework.

## Features

- Create and manage wallets
- Send transactions
- Start a miner server
- View the blockchain
- Reindex UTXO set


## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/your-username/bolk.git
    cd bolk
    ```

2. Install dependencies:
    ```sh
    npm install
    ```

3. Run the development server:
    ```sh
    npm run tauri dev
    ```
    or
    ```sh
    cargo tauri dev
    ```

### Building for Production

To build the application for production, run:
```sh
npm run tauri build
```

###### Creating a Wallet
    1. Open the application.
    2. Navigate to the "Wallet Operations" tab.
    3. Click on "Create New Wallet" to generate a new wallet address.
###### Sending a Transaction
    1. Open the application.
    2. Navigate to the "Transaction Operations" tab.
    3. Fill in the "From Address", "To Address", and "Amount" fields.
    4. Click on "Send Transaction" to initiate the transaction.
###### Starting the Miner Server
    1. Open the application.
    2. Navigate to the "Blockchain Operations" tab.
    3. Enter the port and address.
    4. Click on "Start Mining" to begin mining.
###### Viewing the Blockchain
    1. Open the application.
    2. Navigate to the "Blockchain Operations" tab.
    3. Click on "View Blockchain" to see the current state of the blockchain.
###### Reindexing the UTXO Set
    1. Open the application.
    2. Navigate to the "Transaction Operations" tab.
    3. Click on "Reindex UTXO" to reindex the UTXO set.
Contributing
Contributions are welcome! Please open an issue or submit a pull request.
