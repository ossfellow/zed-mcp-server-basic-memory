# Basic Memory MCP Server Extension for Zed

This extension integrates [Basic Memory](https://github.com/basicmachines-co/basic-memory) as a context server for [Zed's](https://zed.dev) [Assistant](https://zed.dev/docs/assistant/assistant).

Basic Memory lets you build persistent knowledge through natural conversations with Large Language Models, while keeping everything in simple Markdown files on your computer.

## Prerequisites

- Python 3.12+
- [uv](https://github.com/astral-sh/uv) installed

> **⚠️ Important Note**: This extension currently requires Zed to support MCP tools, which is still in development. While you can install the extension now, its functionality will become available once Zed adds MCP tools support in a future update. Follow [Zed's releases](https://github.com/zed-industries/zed/releases) for updates.

## Installation

### 1. Install uv and Basic Memory

```bash
pip install uv
uv tool install basic-memory
```

### 2. Install the extension

Navigate to: **Zed** > **Extensions**. Or use the command palette to search for "extensions".

### 3. Configure the extension

Add the following to your Zed settings:

```json
{
  "context_servers": {
    "mcp-server-basic-memory": {
      "settings": {
        "project": "optional-project-name"
      }
    }
  }
}
```

The `project` setting is optional:

- If specified, Basic Memory will use the named project for storing and accessing notes
- If omitted, Basic Memory will use its default project (typically stored in `~/basic-memory/config.json`, or set in the `BASIC_MEMORY_PROJECT` environment variable)

You can configure these settings either:

- Globally in your Zed settings.json
- Per-project in your project settings
- Or omit them to use Basic Memory's default project

## Usage

Once configured, Basic Memory will be available in the Zed Assistant. You can use prompts like:

- "Create a note about software architecture patterns"
- "What do I know about functional programming?"
- "Search my notes for information about React hooks"

## Multiple Projects

Basic Memory supports multiple projects to separate different kinds of notes. If you work on different codebases or subjects, you might want to maintain separate knowledge bases for each.

## Building the Extension

This extension needs to be compiled to WebAssembly (WASM):

1. **Add the WASM target to your Rust toolchain:**

   ```
   rustup target add wasm32-wasip1
   ```

2. **Build the extension:**

   ```
   cargo build --target wasm32-wasip1 --release
   ```

### Local Testing

For testing locally:

1. Build the extension as described above
2. In Zed, open Extensions (⌘ + Shift + E)
3. Click "Install Dev Extension" and select the extension directory
4. Your dev extension will override any published version with the same name

> Note: When installing as a dev extension, Zed will automatically use the build artifacts from your target directory.

## Basic Memory Resources

For more detailed information about Basic Memory, visit:

- [Basic Memory Documentation](https://memory.basicmachines.co/)
- [GitHub Repository](https://github.com/basicmachines-co/basic-memory)

## License

This project is licensed under the Apache License, Version 2.0 (the "License"). You may obtain a copy of the License at:

    http://www.apache.org/licenses/LICENSE-2.0

This extension interfaces with the `basic-memory` MCP server, which is a separate product developed by [Basic Machines](https://github.com/basicmachines-co) and is licensed under the GNU Affero General Public License, Version 3.0 (AGPL-3.0). The `basic-memory` MCP server's license can be found at:

    https://www.gnu.org/licenses/agpl-3.0.html

Please note that while this extension is distributed under the Apache 2.0 License, if you use the `basic-memory` MCP server through this extension, you must comply with its AGPL-3.0 license terms. This extension itself does not incorporate any code from the `basic-memory` MCP server; it only provides an interface to communicate with it as an external service.
