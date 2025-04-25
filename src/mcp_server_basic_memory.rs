//! Basic Memory MCP Server Extension for Zed
//!
//! This extension integrates Basic Memory's context server with Zed editor through the MCP protocol.
//! It provides project-aware context and memory capabilities by managing the communication between
//! Zed and the Basic Memory server.
//!
//! # Features
//! - Project-specific context management
//! - Integration with Basic Memory through `uvx` command
//! - Configurable project settings
//!
//! # Configuration
//! The extension can be configured through Zed's settings.json or project settings:
//! ```json
//! {
//!   "context_servers": {
//!     "mcp-server-basic-memory": {
//!       "settings": {
//!         "project": "optional-project-name"  // Optional: Specify a project name for context isolation
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! # Requirements
//! - `uvx` command-line tool must be installed (`pip install uv`)

use serde::Deserialize;
use std::process::Command as ProcessCommand;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

// Project is a valid setting for Basic Memory
#[derive(Debug, Deserialize)]
struct BasicMemoryContextServerSettings {
    // Optional project name
    project: Option<String>,
}

// Basic Memory context server extension
struct BasicMemoryModelContextExtension;

impl zed::Extension for BasicMemoryModelContextExtension {
    fn new() -> Self {
        Self
    }

    // Returns the command to run the Basic Memory context server
    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // First, verify that uvx is available
        if !is_uvx_available() {
            return Err("'uvx' command not found. Please install uv: 'pip install uv'".into());
        }

        // Get settings from Zed project configuration
        let settings = ContextServerSettings::for_project("mcp-server-basic-memory", project)?;

        // Create command with standard args structure
        // Important: `--project` must come BEFORE "mcp" in the args list
        let mut args = vec!["basic-memory".to_string()];

        // Add project flag if specified - it must go BEFORE the "mcp" command
        if let Some(settings_value) = settings.settings {
            let settings: BasicMemoryContextServerSettings =
                serde_json::from_value(settings_value).map_err(|e| e.to_string())?;

            if let Some(project_name) = settings.project {
                args.push("--project".to_string());
                args.push(project_name);
            }
        }

        // Add the mcp command AFTER any project specification
        args.push("mcp".to_string());

        // Return the command configuration
        Ok(Command {
            command: "uvx".to_string(),
            args,
            env: vec![], // No environment variables needed in the standard config
        })
    }
}

// Helper function to check if uvx is available
fn is_uvx_available() -> bool {
    ProcessCommand::new("uvx").arg("--version").output().is_ok()
}

zed::register_extension!(BasicMemoryModelContextExtension);
