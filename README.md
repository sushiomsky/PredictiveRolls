# PredictiveRolls

A machine learning-based predictive dice rolling application using neural networks to analyze patterns in provably fair gambling sites.

## ⚠️ Disclaimer

This software is for educational and research purposes only. Gambling involves risk, and you should never gamble with money you cannot afford to lose. The developers are not responsible for any financial losses incurred while using this software.

## Features

- 🧠 AI-powered prediction using transformer-based neural networks
- 🎲 Support for multiple gambling sites (DuckDice, CryptoGames, FreeBitco.in)
- 📊 Multiple betting strategies
- 🔒 Secure API key management
- 🎯 Configurable betting parameters

## Supported Sites

- **DuckDice**: Cryptocurrency dice game
- **CryptoGames**: Multi-crypto gambling platform  
- **FreeBitco.in**: Bitcoin faucet and dice game

## Prerequisites

- Rust 1.70 or later
- Cargo
- GPU with Vulkan support (for neural network inference)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/sushiomsky/PredictiveRolls.git
cd PredictiveRolls
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

1. Copy the example configuration file:
```bash
cp config.toml.example config.toml
```

2. Edit `config.toml` with your API keys and preferences:
```toml
[duck_dice]
enabled = true
api_key = "your_api_key_here"
currency = "BTC"
strategy = "None"
```

### Available Strategies

- `None`: No strategy (default)
- `AiFight`: AI-based fighting strategy
- `BlaksRunner`: Blaks runner strategy
- `MyStrategy`: Custom strategy implementation

## Usage

Run the application:
```bash
cargo run --release
```

The application will:
1. Load your configuration
2. Initialize the neural network model
3. Connect to the configured gambling site
4. Start making predictions and placing bets

## Training the Model

Before running the main application, you need a trained model. The model files should be placed in the configured artifact directory (default: `/home/jvne/Projects/rust/random_guesser/experimental`).

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt --all
```

### Linting
```bash
cargo clippy --all-targets --all-features
```

## Project Structure

```
src/
├── main.rs           # Application entry point
├── config.rs         # Configuration management
├── model.rs          # Neural network model
├── training.rs       # Model training logic
├── inference.rs      # Prediction inference
├── dataset.rs        # Dataset handling
├── data.rs           # Data structures
├── currency.rs       # Currency types
├── util.rs           # Utility functions
├── sites/            # Site-specific implementations
│   ├── duck_dice/    # DuckDice integration
│   ├── crypto_games.rs
│   ├── free_bitco_in.rs
│   └── windice.rs
└── strategies/       # Betting strategies
    ├── ai_fight.rs
    ├── blaks_runner.rs
    ├── my_strategy.rs
    └── none.rs
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Burn](https://github.com/tracel-ai/burn) - A deep learning framework for Rust
- Uses Vulkan backend for GPU acceleration

## Security

**IMPORTANT**: Never commit your `config.toml` file with real API keys or credentials. The file is already added to `.gitignore` to prevent accidental commits.

## Support

For issues, questions, or contributions, please open an issue on GitHub.
