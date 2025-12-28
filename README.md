# Laterem

A command-line tool for streamlining common Docker and Git workflows. Laterem simplifies repetitive development tasks by providing quick commands for managing Docker containers and Git repositories.

## Features

- **Docker Management**: Quick commands to start, stop, and reset Docker Compose environments
- **Git Workflow Automation**: Simplified commit, push, pull, and reset operations with automatic stashing
- **Smart Branch Detection**: Automatically detects and uses your repository's default branch
- **Two Repository Modes**:
  - Standard mode: Operations relative to the default branch
  - Current mode: Operations on the current branch only
- **Safe Operations**: Automatically stages and stashes changes before pulling updates
- **Colorful Terminal Output**: Clear visual feedback for all operations

## Installation

### Prerequisites

- Rust toolchain (cargo)
- Git
- Docker and Docker Compose (for Docker commands)
- sed (for default branch detection)

### Build from Source

```bash
git clone <repository-url>
cd laterem-slab
cargo build --release
```

The binary will be available at `target/release/laterem`.

To install globally:

```bash
cargo install --path .
```

## Usage

```bash
laterem <TARGET> [ACTION] [OPTIONS]
```

### Targets

- `d` or `docker` - Docker Compose operations
- `r` or `repository` - Git repository operations (relative to default branch)
- `rc` or `current` - Git repository operations (on current branch)

### Actions

#### Docker Actions

- `reset` (default) - Down then up (restart containers)
- `down` or `d` - Stop and remove containers
- `up` or `u` - Start containers in detached mode

#### Repository Actions

- `reset` (default) - Stash changes, checkout and pull default branch, return to original branch
- `commit` or `c` - Commit staged changes (requires message via `--args`)
- `push` or `ps` - Push commits to origin
- `pull` or `pl` - Stash changes, pull updates, pop stash

### Options

- `--args <ARGS>` or `-a <ARGS>` - Additional arguments (e.g., commit message)
- `--config <PATH>` or `-c <PATH>` - Config file path (default: `$HOME/.config/laterem/config.json`)
- `--version` - Show version information

## Examples

### Using Short Aliases

You should alias laterem or rename to something shorter like `lat`

```bash
# Docker operations
laterem d          # reset
laterem d u        # up
laterem d d        # down

# Repository operations
laterem r c --args "fix: bug fix"  # commit
laterem r ps                       # push
laterem r pl                       # pull
```

This prevents merge conflicts and keeps your work safe.

## Configuration

Configuration support is planned for future releases. Currently, default settings include:

- Auto-detection of default branch from Git remote
- Stashing enabled for all operations
- Detached mode for Docker containers

## Development

### Project Structure

```
laterem-slab/
├── src/
│   ├── main.rs              # Entry point
│   └── utils/
│       ├── mod.rs           # Module definitions
│       ├── parser.rs        # CLI argument parsing
│       └── entities.rs      # Core logic and types
├── Cargo.toml               # Dependencies
└── README.md
```

### Dependencies

- **clap** - Command-line argument parsing
- **crossterm** - Terminal styling and colors
- **serde** / **serde_json** - Configuration serialization (planned feature)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
