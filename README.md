# Project-Man

A desktop application for managing Git projects with an integrated AI assistant powered by local LLMs (Ollama). Built with Tauri, SvelteKit, and Rust.

![Version](https://img.shields.io/badge/version-0.1.0-blue)

## Features

### 🗂️ Project Management
- Open and manage multiple Git projects simultaneously with a tabbed interface
- Recent projects are persisted locally for quick access
- Custom titlebar with window controls (minimize, maximize, close)

### 🔀 Git Integration
- View staged and unstaged files in your repository
- Inspect file diffs with syntax highlighting for additions and deletions
- Stage/unstage files directly from the UI
- View current branch name

### 🤖 AI-Powered Assistant
- Chat with a local LLM (via Ollama) about your codebase
- AI can read repository files, list directories, search code, and analyze staged diffs
- Tools available to the AI:
  - `read_repo_file` - Read file contents from the repository
  - `list_dir` - List files and directories
  - `search_code` - Search across codebase using ripgrep
  - `read_multiple_files` - Read multiple files at once
  - `get_staged_diff` - Get current staged changes

### ⚙️ Settings & Customization
- Configure Ollama server URL
- Select from available local models
- Customize primary color theme
- Saved messages history

## Tech Stack

| Layer | Technologies |
|-------|-------------|
| **Frontend** | SvelteKit, TypeScript, Vite |
| **Backend** | Rust, Tauri v2 |
| **Database** | libSQL (SQLite) |
| **Git** | git2 crate |
| **AI** | ollama-rs |

## Project Structure

```
project-man/
├── src/                     # SvelteKit frontend
│   ├── lib/
│   │   ├── data_store.ts    # Tauri store for settings
│   │   ├── icons/           # SVG icon components
│   │   └── ui/              # UI components
│   │       ├── chatItem.svelte
│   │       ├── errorToast.svelte
│   │       ├── llmSetting.svelte
│   │       ├── mainView.svelte
│   │       ├── modelDropDown.svelte
│   │       ├── savedMessages.svelte
│   │       └── titlebar.svelte
│   └── routes/              # SvelteKit routes
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── lib.rs           # Main Tauri commands
│   │   ├── main.rs          # Entry point
│   │   └── utils/
│   │       ├── db.rs        # SQLite operations
│   │       ├── git.rs       # Git operations
│   │       ├── l_ollama.rs  # Ollama API integration
│   │       └── ollama_tool.rs # AI tool functions
│   └── tauri.conf.json      # Tauri configuration
├── config/                  # Local configuration
│   └── config.db            # SQLite database
└── static/                  # Static assets
```

## Prerequisites

- **NPM** (v25.0.0)
- **Ollama** (running locally on port 11434)
- **Git**
- **ripgrep (`rg`)** - For code search functionality

## Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/harilet/project-man.git
   cd project-man
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Start Ollama server**
   ```bash
   ollama serve
   ```

4. **Pull a model** (example with llama2)
   ```bash
   ollama pull glm-5.1:cloud
   ```

5. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

6. **Build for production**
   ```bash
   npm run tauri build
   ```

## Usage

### Opening a Project
1. Click the folder icon to browse for a Git repository
2. Or click on a recent project to open it directly

### Viewing Git Changes
1. Switch to the "git" view using the sidebar
2. View staged files in the top panel
3. View unstaged files in the bottom panel
4. Click on any file to see its diff

### Using the AI Assistant
1. Switch to the "chat" view
2. Configure your Ollama server URL and select a model (click "LLM Settings")
3. Ask questions about your codebase
4. The AI has access to your repository and can read files, search code, and analyze changes

## Tauri Commands

| Command | Description |
|---------|-------------|
| `get_staged_files` | Get list of staged files |
| `get_unstaged_files` | Get list of unstaged files |
| `get_file_diff` | Get diff for a specific file |
| `get_unstaged_file_diff` | Get diff for unstaged file |
| `get_all_local_models` | List available Ollama models |
| `get_recent_projects` | Get recently opened projects |
| `set_projects` | Save a project to database |
| `start_ollama_server_check` | Start Ollama server health check |
| `get_current_branch_name` | Get current Git branch |
| `send_message` | Send message to LLM |
| `add_file_index` | Stage a file |
| `remove_file_index` | Unstage a file |

## Configuration

Settings are stored in `settings.json` and database in `config/config.db`:
- **Ollama server URL** - Default: `http://localhost:11434`
- **Selected model** - Your preferred LLM
- **Primary color** - UI theme customization

## Development

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Svelte Extension](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Scripts

```bash
npm run tauri dev  # Run Tauri in development mode
npm run tauri build # Build production binary
```

## Acknowledgments

- [Tauri](https://tauri.app/) - Desktop application framework
- [SvelteKit](https://kit.svelte.dev/) - Frontend framework
- [Ollama](https://ollama.ai/) - Local LLM runtime
- [git2](https://github.com/rust-lang/git2-rs) - Rust Git bindings
