# claude-commit

AI-powered commit message generator built on top of [Claude Code CLI](https://claude.ai/code).

Analyzes your `git diff` and suggests meaningful commit messages — with support for [Conventional Commits](https://www.conventionalcommits.org/), multiple languages, and an interactive selector right in your terminal.

## How it works

1. Checks your `git status` — if there are unstaged changes, lets you pick which files to stage (or skip and use only what's already staged)
2. Reads the staged `git diff`
3. Sends it to Claude Code CLI via `claude --print --output-format json`
4. Parses the response into a list of suggestions
5. Lets you pick one interactively and commits it

## Prerequisites

- [Rust](https://rustup.rs/) (to build from source)
- [Claude Code CLI](https://claude.ai/code) installed and authenticated
  ```sh
  claude login
  ```

## Installation

```sh
git clone https://github.com/axelroman-dev/claude-commit
cd claude-commit
cargo install --path .
```

## Usage

Just run it from inside your repo:

```sh
claude-commit
```

Or use the explicit subcommand:

```sh
claude-commit suggest
```

If you have unstaged changes, you'll get a checklist to pick which files to stage (`Space` to toggle, `Enter` to continue with whatever is checked — or nothing, to keep what's already staged).

Then Claude analyzes the staged diff and shows token usage and cost:

```
tokens  in: 11727 | out: 48 | $0.0260
```

Use arrow keys to select a suggestion and press `Enter` to commit, or `Ctrl+C` to cancel at any point.

### Configuration

Run the interactive setup wizard:

```sh
claude-commit config
```

Or set options directly via flags:

```sh
claude-commit config --language spanish --style conventional --count 3 --max-length 72
```

| Option | Values | Default |
|--------|--------|---------|
| `--language` | `english`, `spanish` | `english` |
| `--style` | `conventional`, `simple` | `conventional` |
| `--count` | `1`–`5` | `3` |
| `--max-length` | min `20` | `80` |

View current configuration:

```sh
claude-commit show
```

Configuration is stored at `~/.config/claude-commit/config.toml`.

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
- [serde_json](https://github.com/serde-rs/json) — parsing Claude CLI output

## License

MIT
