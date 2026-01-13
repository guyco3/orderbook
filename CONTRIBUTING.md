# Contributing to Kalshi Orderbook

First off, thank you for considering contributing! It's people like you who make this a great tool for the prediction market community.

## 🛠️ Development Setup

1. **Clone and Install:**
   ```bash
   git clone https://github.com/guyco3/orderbook
   cd orderbook
   python -m venv .venv
   source .venv/bin/activate
   pip install maturin
   maturin develop
   ```

2. **Run Tests:** Always ensure your changes pass the existing test suite:

   ```bash
   cargo test
   ```

## 📝 Commit Messages

We follow the Conventional Commits specification. This helps us maintain a clean history and automate versioning.

Format: `<type>(<scope>): <description>`

Common Types:

- **feat:** A new feature for the user.
- **fix:** A bug fix.
- **docs:** Documentation only changes.
- **refactor:** A code change that neither fixes a bug nor adds a feature.
- **perf:** A code change that improves performance.
- **chore:** Updating build tasks, package manager configs, etc.

Example: `feat(analysis): add weighted mid-price calculation to analyzer`

## 🚀 Pull Request Process

1. **Create a branch:**
   ```bash
   git checkout -b feat/my-new-feature
   ```

2. **Make your changes and ensure `cargo clippy` is happy.**

3. **Commit using the conventional commit format.**

4. **Open a PR against the `main` branch.**