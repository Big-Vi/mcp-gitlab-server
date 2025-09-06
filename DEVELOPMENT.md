# Working with the MCP SDK

This document explains how to work with the Rust MCP SDK in your development workflow.

## Current Setup

Your workspace now includes:
- `mcp-gitlab-server/` - Your GitLab MCP server implementation  
- `rust-mcp-sdk/` - The official Rust MCP SDK (git submodule)

## The Edition 2024 Issue

The Rust MCP SDK uses Rust edition 2024, which is not yet stable. This means:

1. **Can't use direct dependencies**: You can't add `rmcp` as a dependency in your `Cargo.toml` yet
2. **Can view source code**: You can browse and study the SDK source code in `rust-mcp-sdk/`
3. **Can run examples**: You can build and run SDK examples separately

## How to Study the MCP SDK

### 1. Browse the Source Code
```bash
# Open any file in the SDK
code rust-mcp-sdk/crates/rmcp/src/lib.rs
```

### 2. Check the Examples
```bash
cd rust-mcp-sdk
cargo run --example server-basic
```

### 3. Build the SDK
```bash
cd rust-mcp-sdk
cargo build
cargo test
```

### 4. Read Documentation
```bash
cd rust-mcp-sdk
cargo doc --open
```

## Development Workflow

1. **Study Examples**: Look at `rust-mcp-sdk/examples/servers/` for server implementation patterns
2. **Copy Patterns**: Copy useful patterns to your `mcp-gitlab-server/src/` files
3. **Adapt for GitLab**: Modify the patterns for GitLab-specific functionality
4. **Test Separately**: Build and test your server independently

## When Edition 2024 is Stable

Once Rust edition 2024 is stabilized, you can add the MCP SDK as a dependency:

```toml
[dependencies]
rmcp = { version = "0.6.1", features = ["server", "transport-child-process"] }
rmcp-macros = "0.6.1"
```

## Current Commands

```bash
# Build your GitLab server
cargo build

# Run your GitLab server  
cargo run

# Build the MCP SDK separately
cd rust-mcp-sdk && cargo build

# Run MCP SDK examples
cd rust-mcp-sdk && cargo run --example server-basic
```

## File Structure
```
mcp-gitlab-server/
├── mcp-gitlab-server/     # Your GitLab MCP server
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── rust-mcp-sdk/          # MCP SDK source (read-only reference)
│   ├── crates/rmcp/        # Core MCP library
│   ├── examples/           # Example implementations
│   └── docs/               # Documentation
└── Cargo.toml             # Workspace configuration
```
