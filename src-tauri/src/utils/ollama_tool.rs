use crate::utils::git;
use crate::APP_HANDLE;
use git2::Repository;
use tauri::Emitter;
use walkdir::WalkDir;
use glob::Pattern;
use std::fs::File;
use std::io::Read;

/// Helper function to read a file from the git repository at HEAD
fn read_file_from_repo(repo_location: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    let repo = Repository::open(repo_location)
        .map_err(|e| format!("Failed to open repository at {}: {}", repo_location, e))?;
    let revspec = repo.revparse_single("HEAD")
        .map_err(|e| format!("Failed to parse HEAD: {}", e))?;
    let tree = revspec.peel_to_tree()
        .map_err(|e| format!("Failed to get tree from HEAD: {}", e))?;
    let entry = tree.get_path(std::path::Path::new(file_path))
        .map_err(|e| format!("File '{}' not found in repository: {}", file_path, e))?;
    let blob = repo.find_blob(entry.id())
        .map_err(|e| format!("Failed to get blob for file '{}': {}", file_path, e))?;
    let content = String::from_utf8(blob.content().to_vec())
        .map_err(|e| format!("File '{}' is not valid UTF-8: {}", file_path, e))?;
    
    Ok(content)
}

/// Read the contents of a file in the repository. Use this to understand what changed.
///
/// * file_path - The relative path to the file from the repository root (e.g. "src/main.rs")
/// * repo_location - The absolute location of the repo
#[ollama_rs::function]
pub(crate) async fn read_repo_file(
    file_path: String,
    repo_location: String,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    // Safety: only allow relative paths — prevent escaping the repo
    if file_path.contains("..") || file_path.starts_with('/') {
        // Emit tool execution event to frontend
        if let Some(handle) = APP_HANDLE.get() {
            let _ = handle.emit(
                "tool-execution",
                serde_json::json!({
                    "tool_name": "read_repo_file",
                    "tool_input": file_path,
                    "tool_output": "Only relative paths within the repository are allowed"
                }),
            );
            // Also emit error to app-error event
            let _ = handle.emit("app-error", "Only relative paths within the repository are allowed".to_string());
        }
        return Err("Only relative paths within the repository are allowed".into());
    }

    // Open the repository and get the file content from HEAD
    let content = read_file_from_repo(&repo_location, &file_path)?;

    // Emit completion event
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool_name": "read_repo_file",
                "tool_input": file_path,
                "tool_output": content
            }),
        );
    }
    
    Ok(content)
}

/// List files and directories inside a path in the repository.
/// Returns a tree-like text listing. Use "." for the project root.
///
/// * dir_path - Absolute directory path (Project location + file path)
/// * depth - How many levels deep to list (default 2, max 5)
#[ollama_rs::function]
pub(crate) async fn list_dir(
    dir_path: String,
    depth: Option<u32>,
) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("trying to list dir: {}", dir_path);

    let depth = depth.unwrap_or(2).min(5);
    let base = std::path::Path::new(&dir_path);

    if !base.is_dir() {
        // Emit tool execution event to frontend
        if let Some(handle) = APP_HANDLE.get() {
            let _ = handle.emit(
                "tool-execution",
                serde_json::json!({
                    "tool_name": "list_dir",
                    "tool_input": dir_path,
                    "tool_output": format!("'{}' is not a directory", dir_path)
                }),
            );
            // Also emit error to app-error event
            let _ = handle.emit("app-error", format!("'{}' is not a directory", dir_path));
        }
        return Err(format!("'{}' is not a directory", dir_path).into());
    }

    let mut output = String::new();
    collect_dir_entries(base, base, 0, depth, &mut output);

    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool_name": "list_dir",
                "tool_input": dir_path,
                "tool_output": output
            }),
        );
    }

    Ok(output)
}

pub(crate) fn collect_dir_entries(
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

    // Use current directory as the search root
    let search_root = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    // Collect search options
    let case_sensitive = case_sensitive.unwrap_or(false);
    let glob_pattern = glob.clone();

    // Vector to store all matches
    let mut matches = Vec::new();
    
    // Define directories to ignore
    let ignored_dirs = [
        "target", "vendor", "node_modules", ".git", ".vscode", ".idea", 
        "dist", "build", "out", ".next", ".nuxt", "coverage", "tmp", "temp"
    ];
    
    // Walk the directory tree
    for entry in WalkDir::new(search_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            // Skip directories that should be ignored
            !e.path()
                .components()
                .any(|component| {
                    component.as_os_str().to_str()
                        .map(|s| ignored_dirs.contains(&s))
                        .unwrap_or(false)
                })
        })
        .filter(|e| {
            // Skip hidden files and directories
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            // Apply glob filter if provided
            if let Some(ref pattern_str) = glob_pattern {
                // Use glob crate for proper pattern matching
                if let Ok(pattern) = Pattern::new(pattern_str) {
                    let path_str = e.path().to_string_lossy();
                    pattern.matches(&path_str)
                } else {
                    // If pattern is invalid, include the file
                    true
                }
            } else {
                // Include all files by default
                true
            }
        }) {
        
        // Skip binary files and large files
        if is_binary_file(entry.path()) {
            continue;
        }
        
        // Check file size limit (similar to ripgrep's --max-filesize)
        if let Ok(metadata) = std::fs::metadata(entry.path()) {
            if metadata.len() > 1_000_000 { // 1MB limit
                continue;
            }
        }
        
        // Read and search file content
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            let file_path = entry.path().to_string_lossy();
            
            // Search for query in content
            for (line_number, line) in content.lines().enumerate() {
                let found = if case_sensitive {
                    line.contains(&query)
                } else {
                    line.to_lowercase().contains(&query.to_lowercase())
                };
                
                if found {
                    matches.push(format!("{}:{}:{}", file_path, line_number + 1, line));
                    
                    // Limit total matches to prevent overwhelming the context
                    if matches.len() >= 1000 {
                        break;
                    }
                }
            }
        }
        
        // Early termination if we have too many matches
        if matches.len() >= 1000 {
            break;
        }
    }

    if matches.is_empty() {
        let error_msg = format!("No matches found for '{}'", query);
        // Emit tool execution event to frontend
        if let Some(handle) = APP_HANDLE.get() {
            let _ = handle.emit(
                "tool-execution",
                serde_json::json!({
                    "tool_name": "search_code",
                    "tool_input": format!("{}::{:#?}::{:#?}",query,glob.clone(),case_sensitive,),
                    "tool_output": error_msg.clone()
                }),
            );

            let _ = handle.emit(
                "app-error",
                error_msg.clone(),
            );
        }
        return Ok(error_msg);
    }

    // Truncate to 100 matches to avoid flooding context (same behavior as original)
    let truncated: String = matches.iter().take(100).cloned().collect::<Vec<_>>().join("\n");
    let total = matches.len();

    let result = if total > 100 {
        format!(
            "{}\n... ({} more matches truncated)",
            truncated,
            total - 100
        )
    } else {
        truncated
    };

    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool_name": "search_code",
                "tool_input": format!("{}::{:#?}::{:#?}",query,glob.clone(),case_sensitive,),
                "tool_output": result
            }),
        );
    }

    Ok(result)
}

/// Helper function to determine if a file is binary
fn is_binary_file(path: &std::path::Path) -> bool {
    // Try to read the first few KB of the file to check for null bytes
    // Binary files often contain null bytes, while text files typically don't
    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; 1024]; // Read first 1KB
        if let Ok(bytes_read) = file.read(&mut buffer) {
            // Check for null bytes which are common in binary files
            return buffer[..bytes_read].contains(&0);
        }
    }
    // If we can't read the file, assume it's not binary
    false
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

    let paths: Vec<&str> = file_paths.split(',').map(str::trim).collect();
    let mut result = String::new();

    for file_path in paths {
        if file_path.contains("..") || file_path.starts_with('/') {
            result.push_str(&format!(
                "=== {} ===\nERROR: Only relative paths within the repository are allowed\n\n",
                file_path
            ));
            if let Some(handle) = APP_HANDLE.get() {
            let _ = handle.emit(
                "app-error",
                format!(
                    "=== {} ===\nERROR: Only relative paths within the repository are allowed\n\n",
                    file_path
                ));
            }
            continue;
        }

        result.push_str(&format!("=== {} ===\n", file_path));

        // Try git HEAD first using git2, fall back to working tree
        match read_file_from_repo(".", file_path) {
            Ok(content) => result.push_str(&content),
            Err(_) => match std::fs::read_to_string(file_path) {
                Ok(content) => result.push_str(&content),
                Err(e) => {
                    if let Some(handle) = APP_HANDLE.get() {
                    let _ = handle.emit(
                        "app-error",
                        format!("ERROR: Could not read file: {}", e)
                    );
                    }
                    result.push_str(&format!("ERROR: Could not read file: {}", e))
                },
            },
        }

        result.push_str("\n\n");
    }

    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool_name": "read_multiple_files",
                "tool_input": file_paths,
                "tool_output": result
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
    let diff = match git::get_all_staged_diff(location.clone()) {
        Ok(data) => data,
        Err(e) => {
            format!("error: {}", e)
        }
    };

    // Emit tool execution event to frontend
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "tool-execution",
            serde_json::json!({
                "tool_name": "get_staged_diff",
                "tool_input": location.clone(),
                "tool_output": diff
            }),
        );
    }
    Ok(diff)
}
