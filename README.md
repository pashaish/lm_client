# LM Client

A native desktop client for interacting with Language Models (LLMs) built with Rust and Iced.

## Features

- 💬 Chat conversations with LLMs
- 📂 Conversation organization with folders
- 🔍 RAG (Retrieval Augmented Generation) support
- 🎛️ Multiple AI provider support by OpenAI-Api-Like (OpenAI, Ollama, Gemini, etc.)
- 📦 Preset management for different conversation settings
- 📊 Vector database integration
- 🖥️ Cross-platform (macOS, Windows, Linux)
- 🌙 Dark theme UI

## Screenshots


## Installation

### Pre-built Binaries

Download pre-built binaries for your platform from the [Releases](https://github.com/pashaish/lm_client/releases) page.

### Building from Source

#### Prerequisites

- Rust toolchain (stable, 2024 edition)
- Cargo

#### Build Instructions

Clone the repository:

```sh
git clone https://github.com/pashaish/lm_client.git
cd lm_client
```

Build the project:

```sh
cargo build --release
```

Run the application:

```sh
cargo run --release
```

### Platform-specific Build Scripts

There are several build scripts available in the `scripts` directory:

- `build-mac-arm.sh`: Build for macOS ARM
- `build-windows.sh`: Build for Windows
- `build-linux.sh`: Build for Linux
- `build-bundle.sh`: Create application bundle

## Project Structure

```
lm_client/
├── framework/           # Union functionality for src
├── modules/
│   ├── api/             # API clients for LLM providers
│   ├── database/        # Database implementation
│   ├── services/        # Business logic services
│   ├── types/           # Shared data types and DTOs
│   └── utils/           # Utility functions
├── src/                 # Main application code
│   ├── app/             # Main application state and view
│   ├── theme/           # UI theme definitions
│   └── widgets/         # Custom UI widgets
└── scripts/             # Build and utility scripts
```

## Architecture

The application follows an elm-like architecture using the Iced framework:

- **State**: Manages the application data
- **Update**: Handles events and updates state
- **View**: Renders UI components
- **Subscription**: Manages asynchronous events

The project uses a modular approach with workspaces to separate concerns.

## Contributing

Contributions are welcome!

## Roadmap

See the [todo.md](todo.md) file for planned features and improvements.

Short-term goals:
- Abort Chat Completion handling
- Notification system for errors and tips
- Custom error handling

Long-term goals:
- Custom markdown parser with text selection support
- PDF to text conversion for RAG
- Favorite models functionality

## License

This project is licensed under the [LICENSE](LICENSE) file in the repository root.

## Acknowledgements

- [Iced](https://github.com/iced-rs/iced) - GUI library for Rust
- [SQLite](https://www.sqlite.org/) - Embedded database
