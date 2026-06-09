# commit-ai

AI-powered commit message generator built on top of [Claude Code CLI](https://claude.ai/code).

Analyzes your `git diff` and suggests meaningful commit messages — with support for [Conventional Commits](https://www.conventionalcommits.org/), multiple languages, and an interactive selector right in your terminal.

## How it works

1. Reads your staged (or unstaged) `git diff`
2. Sends it to Claude Code CLI via `claude --print`
3. Parses the response into a list of suggestions
4. Lets you pick one interactively and commits it

## Prerequisites

- [Rust](https://rustup.rs/) (to build from source)
- [Claude Code CLI](https://claude.ai/code) installed and authenticated
  ```sh
  claude login
  ```

## Installation

```sh
git clone https://github.com/your-username/claude-commit
cd claude-commit
cargo install --path .
```

## Usage

Stage your changes, then run:

```sh
commit-ai
```

Or use the explicit subcommand:

```sh
commit-ai suggest
```

Use arrow keys to select a suggestion and press Enter to commit.

### Configuration

Run the interactive setup wizard:

```sh
commit-ai config
```

Or set options directly via flags:

```sh
commit-ai config --language spanish --style conventional --count 3
```

| Option | Values | Default |
|--------|--------|---------|
| `--language` | `english`, `spanish` | `english` |
| `--style` | `conventional`, `simple` | `conventional` |
| `--count` | `1`–`5` | `3` |

View current configuration:

```sh
commit-ai show
```

Configuration is stored at `~/.config/commit-ai/config.toml`.

## Commit styles

**Conventional Commits** (`conventional`)
```
feat(auth): add OAuth2 login flow
fix(api): handle null response from user endpoint
```

**Simple** (`simple`)
```
Add OAuth2 login flow
Fix null response handling in user endpoint
```

## Built with

- [Rust](https://www.rust-lang.org/)
- [Claude Code CLI](https://claude.ai/code) — the AI backbone
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [dialoguer](https://github.com/console-rs/dialoguer) — interactive terminal UI
- [colored](https://github.com/mackwic/colored) — terminal colors

## License

MIT
