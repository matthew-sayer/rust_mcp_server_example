# Rust MCP Server Template (STDIO)

This repository provides a simple example of a **Model Context Protocol (MCP)** server implemented in Rust. **This runs with STDIO.**

The server uses the `prism_mcp_rs` crate to create a basic MCP server with a single tool that greets the user based on their input.

## Features

- **MCP Server**: A lightweight server built using the `prism_mcp_rs` crate.
- **Hello Tool**: A simple tool that takes a user's name as input and responds with a personalised greeting.
- **Asynchronous Execution**: Powered by the `tokio` runtime for handling asynchronous operations.
- **Stdio Transport**: The server communicates using standard input/output (Stdio), making it easy to integrate with other tools or scripts.

## How It Works

1. The server is initialised with a name (`HelloToolServer`) and version (`1.0.0`).
2. A single tool, `hello`, is registered with the server. This tool:
   - Accepts a JSON input with a `name` field.
   - Responds with a greeting message like `Hello, <name>!`.
3. The server uses standard input/output (Stdio) as its transport mechanism for communication.


## Code Overview

### `main.rs`

The `main.rs` file contains the following key components:

- **ToolHandlerCore**: An empty struct implementing the `ToolHandler` trait, which defines the behavior of the `hello` tool. In Rust, we create structs to hold the data to be used, similar to a Python __init__ within a class but without actually initialising anything, only storing the data. Then, we add behaviour to them using implementations, like we've done here for ToolHandler - as opposed to adding functions within a class like in Python. We also have #[derive(Clone)] above the empty struct to allow us to clone this for multiple implementations of tools.
- **MCP Server Initialisation**: The server is created and the `hello` tool is added with its schema and handler.
- **Server Start**: The server is started using the `StdioServerTransport`.

## Adding a tool

Within the main function, write in further tools with:
server.add_tool(toolname, description, json object, struct (toolhandlercore)), and ensure it's awaited or it won't pick up the tool (functions in this server are asynchronous).

### Example Tool: `hello`

The `hello` tool is defined as follows:
- **Input Schema**:
  ```json
  {
      "type": "object",
      "properties": {
          "name": {
              "type": "string",
              "description": "The name of the person to greet."
          }
      }
  }

```rust
    server.add_tool(
        "hello".to_string(), // NAME
        Some("Says hello to the specified user.".to_string()), // DESCRIPTION
        json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The name of the person to greet."
                }
            }
        }), // JSON INPUT SCHEMA
        ToolHandlerCore, // CORE STRUCT
    ).await?;
```
## Dependencies 
Everything in the Cargo.toml, and an installation of CMake.

## How to use it?
Build it with cargo build --release, and then cargo run --release. It will then create a release folder within a "target" folder where you can target the exe within there as your STDIO server to use.

## Example mcp.json for use with VSCode
```json
{
  "servers": {
    "RustMCPServer": {
      "type": "stdio",
      "command": "target\\release\\rust_mcp_template.exe",
      "args": ["--mode", "vscode"],
      "env": {
        "VSCODE_WORKSPACE": "${workspaceFolder}"
      }
    }
  }
}
