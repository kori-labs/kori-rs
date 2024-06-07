# Kori MEV SDK

Kori SDK is a Rust library for interacting with the Kori MEV Block Engine and Searcher services. 
It provides functionalities for authentication, streaming mempool transactions, subscribing to bundles, and sending bundles.

## Features

- **Authentication**: Authenticate using your private key to obtain access and refresh tokens.
- **Streaming Mempool Transactions**: Stream transactions from the client to the Kori MEV Block Engine.
- **Subscribe to Bundles**: Subscribe to receive a stream of simulated and profitable bundles.
- **Send Bundles**: Send bundles to the Kori MEV Block Engine for processing.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
kori_mev_sdk = { git = "https://github.com/yourusername/kori_mev_sdk" }
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with any changes or enhancements. Follow these steps to contribute:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes and commit them with clear and descriptive messages.
4. Push your changes to your fork.
5. Open a pull request to the main repository.
