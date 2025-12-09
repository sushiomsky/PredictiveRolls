# PredictiveRolls

A machine learning-based predictive dice rolling application using neural networks to analyze patterns in provably fair gambling sites.

**Available for Desktop (Linux/Windows/macOS) and Android!**

## ⚠️ Disclaimer

This software is for educational and research purposes only. Gambling involves risk, and you should never gamble with money you cannot afford to lose. The developers are not responsible for any financial losses incurred while using this software.

## Features

- 🧠 AI-powered prediction using transformer-based neural networks
- 🎲 Support for multiple gambling sites (DuckDice, CryptoGames, FreeBitco.in)
- 📊 Multiple betting strategies
- 🔒 Secure API key management
- 🎯 Configurable betting parameters
- 📱 **Android app** with native performance via JNI

## Supported Sites

- **DuckDice**: Cryptocurrency dice game
- **CryptoGames**: Multi-crypto gambling platform  
- **FreeBitco.in**: Bitcoin faucet and dice game

## Prerequisites

### Desktop Version
- Rust 1.70 or later
- Cargo
- GPU with Vulkan support (for neural network inference)

### Android Version
- Android Studio and Android SDK
- Android NDK (r25 or later)
- Rust toolchain with Android targets
- See [android/README.md](android/README.md) for detailed setup

## Installation

### Desktop Installation

1. Clone the repository:
```bash
git clone https://github.com/sushiomsky/PredictiveRolls.git
cd PredictiveRolls
```

2. Build the project:
```bash
cargo build --release
```

### Android Installation

See the complete Android build instructions in [android/README.md](android/README.md).

Quick start:
```bash
cd android
./build.sh
./gradlew assembleDebug
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

3. (Optional) Set up environment variables:
```bash
cp .env.example .env
```

Edit `.env` to customize:
- `CONFIG_PATH`: Path to your config file (default: `config.toml`)
- `MODEL_DIR`: Directory containing trained model files
- `RUST_LOG`: Logging level (`trace`, `debug`, `info`, `warn`, `error`)

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

Or with custom environment variables:
```bash
export MODEL_DIR=/path/to/your/model
export CONFIG_PATH=my_config.toml
export RUST_LOG=debug
cargo run --release
```

The application will:
1. Load your configuration
2. Initialize the neural network model
3. Connect to the configured gambling site
4. Start making predictions and placing bets

### Logging

The application uses `env_logger` for logging. Set the `RUST_LOG` environment variable to control verbosity:
- `RUST_LOG=error` - Only errors
- `RUST_LOG=warn` - Warnings and errors
- `RUST_LOG=info` - Informational messages (default)
- `RUST_LOG=debug` - Debug information
- `RUST_LOG=trace` - Verbose trace information

## Training the Model

Before running the main application, you need a trained model. The model files should be placed in the configured artifact directory. You can specify the location using the `MODEL_DIR` environment variable.

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
