

use async_trait::async_trait;
use serde_json::json;
use tracing::info;

use loongclaw_core::llm_types::ToolDefinition;
use super::{schema_object, Tool, ToolResult, servicor};

pub struct BrowserTool {
    default_timeout_secs: u64,
    browser_executable_path: Option<String>,
}

fn split_browser_command(command: &str) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for ch in command.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if let Some(q) = quote {
            if ch == q {
                quote = None;
            } else {
                current.push(ch);
            }
            continue;
        }

        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            continue;
        }

        if ch.is_whitespace() {
            if !current.is_empty() {
                args.push(current.clone());
                current.clear();
            }
            continue;
        }

        current.push(ch);
    }

    if escaped {
        current.push('\\');
    }
    if quote.is_some() {
        return Err("unclosed quote".into());
    }
    if !current.is_empty() {
        args.push(current);
    }
    Ok(args)
}

impl BrowserTool {
    pub fn new(_data_dir: &str, browser_executable_path: Option<String>) -> Self {
        BrowserTool {
            default_timeout_secs: 30,
            browser_executable_path,
        }
    }

    pub fn with_default_timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.default_timeout_secs = timeout_secs;
        self
    }



    fn get_browser_executable(&self) -> String {
        if let Some(path) = &self.browser_executable_path {
            return path.clone();
        }
        
        // Default browser executable path
        if cfg!(target_os = "linux") {
            "/usr/bin/chromium-browser".to_string()
        } else if cfg!(target_os = "windows") {
            "C:/Program Files/Google/Chrome/chrome.exe".to_string()
        } else if cfg!(target_os = "macos") {
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome".to_string()
        } else {
            "chromium-browser".to_string()
        }
    }
}

#[async_trait]
impl Tool for BrowserTool {
    fn name(&self) -> &str {
        "browser"
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "browser".into(),
            description: "Headless browser automation. Browser state (cookies, localStorage, login sessions) persists across calls and across conversations.\n\n 
                ## Basic workflow\n\
                1. `open <url>` — navigate to a URL\n\
                2. `snapshot -i` — get interactive elements with refs (@e1, @e2, ...)\n\
                3. `click @e1` / `fill @e2 \"text\"` — interact with elements\n\
                4. `get text @e3` — extract text content\n\
                5. Always run `snapshot -i` after navigation or interaction to see updated state\n\n\
                ## All available commands\n\
                **Navigation**: open, back, forward, reload, close\n\
                **Interaction**: click, dblclick, fill, type, press, hover, select, check, uncheck, upload, drag\n\
                **Scrolling**: scroll <dir> [px], scrollintoview <sel>\n\
                **Data extraction**: get text/html/value/attr/title/url/count/box <sel>\n\
                **State checks**: is visible/enabled/checked <sel>\n\
                **Snapshot**: snapshot (-i for interactive only, -c for compact)\n\
                **Screenshot/PDF**: screenshot [path] (--full for full page), pdf <path>\n\
                **JavaScript**: eval <js>\n\
                **Cookies**: cookies, cookies set <name> <val>, cookies clear\n\
                **Storage**: storage local [key], storage local set <k> <v>, storage local clear (same for session)\n\
                **Tabs**: tab, tab new [url], tab <n>, tab close [n]\n\
                **Frames**: frame <sel>, frame main\n\
                **Dialogs**: dialog accept [text], dialog dismiss\n\
                **Viewport**: set viewport <w> <h>, set device <name>, set media dark/light\n\
                **Network**: network route <url> [--abort|--body <json>], network requests\n\
                **Wait**: wait <sel|ms|--text|--url|--load|--fn>\n\
                **Auth state**: state save <path>, state load <path>\n\
                **Semantic find**: find role/text/label/placeholder <value> <action> [input]".into(),
            input_schema: schema_object(
                json!({
                    "command": {
                        "type": "string",
                        "description": "The browser command to run (e.g. `open https://example.com`, `snapshot -i`, `fill @e2 \"hello\"`)"
                    },
                    "timeout_secs": {
                        "type": "integer",
                        "description": "Timeout in seconds (defaults to configured tool timeout budget)"
                    }
                }),
                &["command"],
            ),
        }
    }

    async fn execute(&self, input: serde_json::Value) -> ToolResult {
        let command = match input.get("command").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => return ToolResult::error("Missing 'command' parameter".into()),
        };

        let _timeout_secs = input
            .get("timeout_secs")
            .and_then(|v| v.as_u64())
            .unwrap_or(self.default_timeout_secs);

        let command_args = match split_browser_command(command) {
            Ok(parts) if !parts.is_empty() => parts,
            Ok(_) => return ToolResult::error("Empty browser command".into()),
            Err(e) => {
                return ToolResult::error(format!(
                    "Invalid browser command syntax (quote parsing failed): {e}"
                ));
            }
        };

        let program = self.get_browser_executable();
        info!("Executing browser command via '{}'", program);

        // Handle different command types using servicor
        if !command_args.is_empty() {
            match command_args[0].as_str() {
                "open" if command_args.len() > 1 => {
                    // For open command, use servicor's Visit function
                    let url = command_args[1].clone();
                    servicor::visit(&url);
                    ToolResult::success("Visit executed successfully. Results are printed to console.".into())
                },
                "close" => {
                    // For close command, not supported by servicor
                    return ToolResult::error("Close command not supported.".into());
                },
                _ => {
                    // For other commands, not supported by servicor
                    return ToolResult::error("This command is not supported by servicor.".into());
                }
            }
        } else {
            return ToolResult::error("Empty browser command".into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_split_browser_command() {
        let args = split_browser_command("fill @e2 \"hello world\"").unwrap();
        assert_eq!(args, vec!["fill", "@e2", "hello world"]);
    }

    #[test]
    fn test_split_browser_command_unclosed_quote() {
        let err = split_browser_command("open \"https://example.com").unwrap_err();
        assert!(err.contains("unclosed quote"));
    }

    #[test]
    fn test_browser_tool_name_and_definition() {
        let tool = BrowserTool::new("/tmp/test-data", None);
        assert_eq!(tool.name(), "browser");
        let def = tool.definition();
        assert_eq!(def.name, "browser");
        assert!(def.description.contains("browser"));
        assert!(def.description.contains("cookies"));
        assert!(def.description.contains("eval"));
        assert!(def.description.contains("pdf"));
        assert!(def.input_schema["properties"]["command"].is_object());
        assert!(def.input_schema["properties"]["timeout_secs"].is_object());
    }



    #[tokio::test]
    async fn test_browser_missing_command() {
        let tool = BrowserTool::new("/tmp/test-data", None);
        let result = tool.execute(json!({})).await;
        assert!(result.is_error);
        assert!(result.content.contains("Missing 'command"));
    }
}
