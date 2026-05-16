# send_glitch

## Project Overview
`send_glitch` is a Rust command-line application that reads JSON lines from standard input and sends them as formatted messages to a Matrix room via the Matrix protocol (`matrix-sdk`).

- **Architecture/Tech Stack**: Rust, Tokio (async runtime), `matrix-sdk` (Matrix interaction), Serde (JSON/YAML parsing), and `stdinix` (stdin streaming).
- **Core Workflow**: The app reads a configuration file (default: `config.yaml`), authenticates with a Matrix server, finds a matching room by alias, and then continuously reads JSON lines from stdin. It extracts an HTML snippet (based on a configurable key) from each JSON object and sends it as a `text_html` message to the Matrix room.

## Building and Running
The project is built using standard Cargo tooling. A `justfile` is provided for common development workflows.

- **Build**: `cargo build`
- **Run**: `cargo run -- [path_to_config.yaml]` (Defaults to `config.yaml` in the current directory if no argument is provided)
- **Usage Example**:
  ```bash
  echo '{ "html": "<a href=\"https://google.com\">My google link</a>" }' | cargo run
  ```

### Just Commands
You can run `just <command>` to execute predefined tasks:
- `just check`: Runs the full suite of checks (`update`, `clippy`, `cargo-check`, `fmt`, `test`, `outdated`).
- `just clippy`: Runs the linter (`cargo clippy`).
- `just fmt`: Formats code (`cargo fmt`).
- `just test`: Runs tests (`cargo test`).
- `just outdated`: Checks for outdated dependencies (`cargo outdated -R`).

## Configuration
The application requires a YAML configuration file containing the following required fields:
- `password`: Account password
- `room`: Target room alias
- `account`: Matrix user account (e.g., `@username:matrix.org`)
- `html_json_key`: The key in the incoming JSON to extract the HTML content from.

## Development Conventions & Constraints
- **Formatting and Linting**: The project relies on `cargo fmt` and `cargo clippy`. The CI pipeline enforces clippy warnings (`-D warnings`). Always format and lint code before committing.
- **Error Handling**: Uses `eyre` for returning `Result` types and the `oops` crate for convenient error attachment.
- **Known Issues / Toolchain Constraints**: 
  - **IMPORTANT**: The Rust toolchain is currently pinned to `1.93.1` in `rust-toolchain.toml`. This is to work around a Rust 1.94+ regression causing "queries overflow the depth limit!" in the `matrix-sdk` crate. Do **not** update the Rust version until `matrix-sdk` > 0.16.0 is released and confirmed to fix the issue.