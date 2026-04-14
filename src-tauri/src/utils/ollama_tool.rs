use crate::utils::git;
use crate::APP_HANDLE;
use std::process::Command;
use tauri::Emitter;

/// Read the contents of a file in the repository. Use this to understand what changed.
///
/// * file_path - The relative path to the file from the repository root (e.g. "src/main.rs")
#[ollama_rs::function]
pub(crate) async fn read_repo_file(
    file_path: String,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool": "read_repo_file",
                "data": file_path
            }),
        );
    }
    // Safety: only allow relative paths — prevent escaping the repo
    if file_path.contains("..") || file_path.starts_with('/') {
        return Err("Only relative paths within the repository are allowed".into());
    }

    let output = Command::new("git")
        .args(["show", &format!("HEAD:{}", file_path)])
        .output()?;

    if !output.status.success() {
        // Fall back to reading from the working tree
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Could not read file '{}': {}", file_path, e))?;

        // Emit completion event
        if let Some(handle) = APP_HANDLE.get() {
            let _ = handle.emit(
                "tool-execution",
                serde_json::json!({
                    "tool": "read_repo_file",
                    "data": content,
                }),
            );
        }

        return Ok(content);
    }

    let result = String::from_utf8(output.stdout)?;

    // Emit completion event
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool": "read_repo_file",
                "data": file_path,
            }),
        );
    }

    Ok(result)
}

/// List files and directories inside a path in the repository.
/// Returns a tree-like text listing. Use "." for the project root.
///
/// * dir_path - Relative directory path (e.g. "src" or ".")
/// * depth - How many levels deep to list (default 2, max 5)
#[ollama_rs::function]
pub(crate) async fn list_dir(
    dir_path: String,
    depth: Option<u32>,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("trying to list dir: {}", dir_path);

    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool": "list_dir",
                "data": dir_path,
            }),
        );
    }

    if dir_path.contains("..") || dir_path.starts_with('/') {
        return Err("Only relative paths within the repository are allowed".into());
    }

    let depth = depth.unwrap_or(2).min(5);
    let base = std::path::Path::new(&dir_path);

    if !base.is_dir() {
        return Err(format!("'{}' is not a directory", dir_path).into());
    }

    let mut output = String::new();
    collect_dir_entries(base, base, 0, depth, &mut output);

    // Emit completion event
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool": "list_dir",
                "data": dir_path,
            }),
        );
    }

    Ok(output)
}

fn collect_dir_entries(
    dir: &std::path::Path,
    root: &std::path::Path,
    depth: u32,
    max_depth: u32,
    output: &mut String,
) {
    if depth > max_depth {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    let mut items: Vec<_> = entries.flatten().collect();
    items.sort_by_key(|e| e.file_name());

    for entry in items {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }

        let indent = "  ".repeat(depth as usize);

        if path.is_dir() {
            output.push_str(&format!("{}{}/\n", indent, name));
            collect_dir_entries(&path, root, depth + 1, max_depth, output);
        } else {
            let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            output.push_str(&format!("{}{} ({} bytes)\n", indent, name, size));
        }
    }
}

/// Search for a pattern across the codebase using ripgrep.
/// Returns matching lines with file paths and line numbers.
///
/// * query - Regex or literal string to search for
/// * glob - Optional file glob filter, e.g. "*.rs" or "src/**/*.ts"
/// * case_sensitive - Whether to match case-sensitively (default false)
#[ollama_rs::function]
pub(crate) async fn search_code(
    query: String,
    glob: Option<String>,
    case_sensitive: Option<bool>,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("searching code for: {}", query);

    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit("tool-execution", serde_json::json!({
            "tool": "search_code",
            "data": format!("{}\n{:#?}\n{:#?}",query,glob.clone(),case_sensitive,),
        }));
    }

    let mut cmd = std::process::Command::new("rg");
    cmd.arg("--line-number")
        .arg("--with-filename")
        .arg("--color=never")
        .arg("--max-filesize=1M");

    if !case_sensitive.unwrap_or(false) {
        cmd.arg("--ignore-case");
    }

    if let Some(ref g) = glob {
        cmd.arg("--glob").arg(g);
    }

    cmd.arg(&query);

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run ripgrep: {}. Is 'rg' installed?", e))?;

    let stdout = String::from_utf8(output.stdout)?;

    if stdout.is_empty() {
        return Ok(format!("No matches found for '{}'", query));
    }

    // Truncate to 100 matches to avoid flooding context
    let truncated: String = stdout.lines().take(100).collect::<Vec<_>>().join("\n");
    let total = stdout.lines().count();

    let result = if total > 100 {
        format!(
            "{}\n... ({} more matches truncated)",
            truncated,
            total - 100
        )
    } else {
        truncated
    };

    // Emit completion event
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool": "search_code",
                "data": total,
            }),
        );
    }

    Ok(result)
}

/// Read several files in one call. Prefer this over multiple read_repo_file calls.
/// Returns each file's content separated by headers, or an error per file if unreadable.
///
/// * file_paths - Comma-separated list of relative file paths, e.g. "src/main.rs,src/lib.rs"
#[ollama_rs::function]
pub(crate) async fn read_multiple_files(
    file_paths: String,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("trying to read files: {}", file_paths);

    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool": "read_multiple_files",
                "data": file_paths
            }),
        );
    }

    let paths: Vec<&str> = file_paths.split(',').map(str::trim).collect();
    let mut result = String::new();

    for file_path in paths {
        if file_path.contains("..") || file_path.starts_with('/') {
            result.push_str(&format!(
                "=== {} ===\nERROR: Only relative paths within the repository are allowed\n\n",
                file_path
            ));
            continue;
        }

        result.push_str(&format!("=== {} ===\n", file_path));

        // Try git HEAD first, fall back to working tree (same logic as read_repo_file)
        let output = std::process::Command::new("git")
            .args(["show", &format!("HEAD:{}", file_path)])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                result.push_str(&String::from_utf8(out.stdout)?);
            }
            _ => match std::fs::read_to_string(file_path) {
                Ok(content) => result.push_str(&content),
                Err(e) => result.push_str(&format!("ERROR: Could not read file: {}", e)),
            },
        }

        result.push_str("\n\n");
    }

    // Emit completion event
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool": "read_multiple_files",
                "data": file_paths,
            }),
        );
    }

    Ok(result)
}

/// Get the staged diff (git index vs HEAD) for the repository at the given location.
/// Returns the diff in patch format, with each line prefixed by its origin character
/// ('+' for added, '-' for removed, ' ' for context). Use this to understand what
/// changes are currently staged before writing a commit message or reviewing edits.
///
/// * location - Absolute path to the root of the git repository
#[ollama_rs::function]
pub(crate) async fn get_staged_diff(
    location: String,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool": "get_staged_diff",
                "data": location.clone()
            }),
        );
    }

    let diff = match git::get_all_staged_diff(location.clone()) {
        Ok(data) => {
            // Emit completion event
            if let Some(handle) = APP_HANDLE.get() {
                let _ = handle.emit(
                    "tool-execution",
                    serde_json::json!({
                        "tool": "get_staged_diff",
                        "data": location,
                    }),
                );
            }
            data
        }
        Err(e) => {
            // Emit error event
            if let Some(handle) = APP_HANDLE.get() {
                let _ = handle.emit(
                    "tool-execution",
                    serde_json::json!({
                        "tool": "get_staged_diff",
                        "data": e.to_string()
                    }),
                );
            }
            format!("error: {}", e)
        }
    };
    Ok(diff)
}
