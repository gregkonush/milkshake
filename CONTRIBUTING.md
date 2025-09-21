# Contributing to Milkshake

👋 Thanks for your interest in improving Milkshake! This project empowers developers to generate Nano Banana (Gemini 2.5 Flash Image Preview) artwork from the command line. Contributions are welcome—bug reports, feature ideas, docs updates, or code patches.

## Getting Started

1. Fork this repository and clone your fork.
2. Ensure you have Rust 1.80 or newer installed (`rustup update` recommended).
3. Install project dependencies (no extra tooling beyond Cargo is required).
4. Run the standard developer workflow:
   ```bash
   cargo fmt
   cargo build
   cargo test
   ```

## Development Guidelines

- Keep the clipboard-first experience in mind—features should not break `milkshake "prompt"` on mainstream platforms.
- Prefer small, focused pull requests with clear rationale in the description.
- Add tests when fixing bugs or adding behaviour. When API calls are involved, stub or document how to reproduce manually.
- Follow Rust 2024 idioms and keep modules commented only where logic is non-obvious.
- Run `cargo fmt` before submitting. If you have `cargo clippy` installed, running it locally is encouraged but not required.

## Documentation

- Update `README.md` for user-facing changes.
- Append entries to `CHANGELOG.md` under the *Unreleased* section.
- Reference any new command-line flags or environment variables in the *Configuration* table.

## Pull Request Checklist

- [ ] Tests pass (`cargo test`).
- [ ] Code is formatted (`cargo fmt`).
- [ ] README/CHANGELOG updated (when applicable).
- [ ] Screenshots or GIFs attached for UX-affecting changes (optional but helpful).

## Release Process

When cutting a new release:

1. Bump the version in `Cargo.toml`.
2. Update `CHANGELOG.md` with the release date and highlights.
3. Run the full build and test flow and ensure `cargo publish --dry-run` succeeds.
4. Tag the commit (`git tag vX.Y.Z`) and push (`git push --tags`).
5. Draft a GitHub release pointing to the changelog entry.

## Code of Conduct

This project adheres to the [Contributor Covenant](./CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

Thanks again for contributing—have fun blending your prompts into Milkshake visuals! 🍨
