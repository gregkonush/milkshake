# milkshake

Milkshake is an open-source CLI that turns plain text prompts into images using Google's Nano Banana (Gemini 2.5 Flash Image Preview) model, then ships the PNG straight to your clipboard and disk. It is designed to feel invisible—developers type one command and instantly have a shareable visual ready for social, docs, or prototypes.

## Features

- **Prompt → PNG in one command** – Send prompts to Nano Banana with a single CLI invocation.
- **Clipboard-first UX** – Every generated image is copied to the system clipboard for frictionless pasting.
- **Smart file management** – Images are saved into `~/Pictures/milkshake/` (or a provided path) with UUID filenames.
- **Robust safety surfacing** – Safety ratings from Gemini are echoed so you know when moderation rules applied.
- **Infra-friendly** – Pure Rust binary, no runtime dependencies, ships with tests and MIT license.

## Quickstart

1. **Grab an API key** – Visit [Google AI for Developers](https://ai.google.dev/) and ensure your key has access to the Nano Banana (`gemini-2.5-flash-image-preview`) model.
2. **Export the key**:

   ```bash
   export GOOGLE_API_KEY="your-key"
   ```

3. **Install Milkshake from source** (until a crates.io release is available):

   ```bash
   cargo install --path .
   ```

   Or build a release binary directly:

   ```bash
   cargo build --release
   ```

4. **Generate your first image**:

   ```bash
   milkshake "A Gemini-branded nano banana dessert plated on a futuristic table"
   ```

Milkshake exits immediately if no prompt is provided and nothing is piped on STDIN, so be sure to pass text (or `echo "prompt" | milkshake`).

## Configuration

| Option | Description | Default |
| --- | --- | --- |
| `GOOGLE_API_KEY` | API key used for authentication. | required |
| `MILKSHAKE_MODEL` / `-m, --model` | Gemini model identifier. | `gemini-2.5-flash-image-preview` |
| `--api-key <KEY>` | Override the environment variable inline. | – |
| `-o, --output <PATH>` | Write the image to an explicit path. | `~/Pictures/milkshake/milkshake-<uuid>.png` (with sensible fallbacks) |
| `--no-copy` | Skip copying to the clipboard. | Clipboard enabled |
| `--timeout <SECONDS>` | Request timeout window. | `60` |
| `-q, --quiet` | Silence informational output. | Verbose |
| `--format <FORMAT>` | Output format (PNG today; extensible later). | `png` |

### Clipboard behaviour

- macOS & Windows: works out of the box via [arboard](https://crates.io/crates/arboard).
- Linux: requires an active X11/Wayland session; in headless mode, pass `--no-copy`.

### Output locations

Milkshake prefers your Pictures directory (`~/Pictures/milkshake/`). If it cannot create that folder it will fall back to `./milkshake/` inside the current working directory. Supply `--output` to control the exact filename.

## Troubleshooting

| Symptom | Fix |
| --- | --- |
| `Gemini API error ... response_mime_type` | You are likely hitting the REST API manually. Milkshake already requests the correct modalities; ensure your key has image-generation access. |
| CLI appears to hang | Running `milkshake` with no prompt waits for STDIN. Provide a prompt argument or pipe data. |
| Clipboard unavailable warning | Ensure you have clipboard access (Linux desktop session) or use `--no-copy`. |

## Development

- Required toolchain: Rust 1.80+ (edition 2024).
- Formatting & linting:

  ```bash
  cargo fmt
  ```

- Build & test:

  ```bash
  cargo build
  cargo test
  ```

- Create a release build:

  ```bash
  cargo build --release
  ```

Before opening a PR, run the commands above and ensure the README, CHANGELOG, and version number are updated as appropriate.

## Roadmap

- Additional output formats (WebP, JPEG) when Gemini exposes them.
- Aspect-ratio and style presets once public parameters are available.
- Parallel prompt batching for campaign workflows.

## License

MIT © 2025 Greg Konush
