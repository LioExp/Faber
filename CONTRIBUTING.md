# Contributing to Faber

Thank you for your interest in contributing! 🎉

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code.

## How can I contribute?

- Report bugs using the bug issue template
- Suggest features with the feature request template
- Improve documentation
- Submit pull requests

## Setting up the development environment

1. **Install Rust** (if you don't have it yet):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install system dependencies**:
   - Ubuntu/Debian: `sudo apt install build-essential cmake`
   - Arch/Hyprland: `sudo pacman -S base-devel cmake`

3. **Clone the repository**:
   ```bash
   git clone https://github.com/LioExp/Faber.git
   cd Faber
   ```

4. **Build the project**:
   ```bash
   cargo build
   ```
   ⚠️ **The first build takes 10–20 minutes** because it compiles `llama.cpp` from scratch. Subsequent builds are fast.

## Before submitting a Pull Request

- `cargo fmt` — formats the code automatically
- `cargo clippy -- -D warnings` — checks for bad practices
- `cargo test` — runs all tests
- Test with WiFi turned off if your change affects indexing or generation
- Check that RAM usage is within the budget (see [PRD](docs/PRD.md))

## Project structure

- `src/` — Rust source code
  - `main.rs` — entry point
  - `cli.rs` — CLI command definitions (clap)
  - `indexer.rs` — note indexing
  - `retriever.rs` — semantic search
  - `generator.rs` — response generation
  - `config.rs` — configuration management
  - `models.rs` — model download
  - `db.rs` — SQLite interaction
  - `error.rs` — error handling
  - `utils.rs` — helper functions
- `tests/` — integration tests
- `test_data/` — test data
- `docs/` — documentation
  - `pt/` — Portuguese documentation
  - `PRD.md` — product requirements
  - `ARCHITECTURE.md` — architecture design
  - `MODELOS.md` — supported models guide

## Reporting bugs

Use the bug template in `.github/ISSUE_TEMPLATE/bug_report.md`, [bug_report](.github/ISSUE_TEMPLATE/bug_report.md).Always include information about your hardware (RAM, CPU) and whether you were offline.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE) of the project.
```
