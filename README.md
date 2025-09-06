# GitLab MCP Server

A Model Context Protocol (MCP) server for GitLab integration, built with Rust.

## Project Structure

This workspace contains:

- `mcp-gitlab-server/` - Your GitLab MCP server implementation
- `rust-mcp-sdk/` - The official Rust MCP SDK (git submodule)

## Setup

1. **Clone with submodules** (if not already done):
   ```bash
   git submodule update --init --recursive
   ```

2. **Build the GitLab MCP server**:
   ```bash
   cargo build
   ```

3. **Run the GitLab MCP server**:
   ```bash
   cargo run --package mcp-gitlab-server
   # or
   cd mcp-gitlab-server && cargo run
   ```

4. **Watch for changes**:
   ```bash
   cargo watch -x "run --package mcp-gitlab-server"
   # or
   cd mcp-gitlab-server && cargo watch -x run
   ```

## Development

### Working with the MCP SDK

The Rust MCP SDK is included as a git submodule in the `rust-mcp-sdk/` directory. You can:

- **Browse the source code** in VS Code by opening files in `rust-mcp-sdk/`
- **Check examples** in `rust-mcp-sdk/examples/` for implementation patterns
- **Read documentation** in `rust-mcp-sdk/README.md`

### Building the MCP SDK separately

The MCP SDK has its own workspace and can be built independently:

```bash
cd rust-mcp-sdk
cargo build
cargo test
```


## Contributing

1. Make sure both the main project and submodule are up to date:
   ```bash
   git pull
   git submodule update --remote
   ```

2. Make your changes in the `mcp-gitlab-server/` directory

3. Test your changes:
   ```bash
   cargo test
   cargo check
   ```

4. Commit and push your changes

## Updating the MCP SDK

To update the MCP SDK submodule to the latest version:

```bash
cd rust-mcp-sdk
git pull origin main
cd ..
git add rust-mcp-sdk
git commit -m "Update rust-mcp-sdk submodule"
```

cargo watch -x "run --package mcp-gitlab-server"

rustup update
rustup install 1.86.0
rustc --version
cargo run --example servers_counter_streamhttp
npx modelcontextprotocol/inspector#0.16.2

cargo expand --package mcp-gitlab-server --lib

cargo expand --package mcp-gitlab-server --bin mcp-gitlab-server > expanded_gitlab.rs

cargo expand --package mcp-gitlab-server --lib > expanded_gitlab.rs

code expanded_gitlab.rs