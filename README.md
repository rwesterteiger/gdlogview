# logview — NDJSON Log File TUI Viewer

A Ratatui-based terminal UI for browsing NDJSON log files in a table layout with
filtering and multi-line expansion.

## Build & Run

```bash
cargo build --release
./target/release/logview path/to/logfile.ndjson
```

Or directly:

```bash
cargo run -- path/to/logfile.ndjson
```

## Features

- **Table view** with columns: Time, Level (color-coded), Logger, Message
- **Logger filter** (`l` key): substring search — e.g. typing `SysInfo` matches `L_SysInfo`
- **Message filter** (`/` key): substring search on the message text
- **Multi-line folding**: messages with embedded `\n` show a `▶` marker when collapsed
  and `▼` when expanded; press **Space** to toggle
- Filters update live as you type

## Keybindings

### Table Mode

| Key            | Action                        |
|----------------|-------------------------------|
| `q`            | Quit                          |
| `↑` / `k`     | Previous row                  |
| `↓` / `j`     | Next row                      |
| `g`            | Jump to first row             |
| `G`            | Jump to last row              |
| `PgUp`         | Page up                       |
| `PgDn`         | Page down                     |
| `Space`        | Expand / collapse multi-line  |
| `l`            | Focus logger filter           |
| `/`            | Focus message filter          |
| `Tab`          | Focus logger filter           |
| `Ctrl+C`       | Quit (any mode)               |

### Filter Mode

| Key            | Action                        |
|----------------|-------------------------------|
| Type           | Updates filter live            |
| `Backspace`    | Delete character               |
| `Enter`        | Apply and return to table      |
| `Esc`          | Return to table                |
| `Tab`          | Switch between filter fields   |

## Expected NDJSON Format

Each line must be a JSON object with these fields:

```json
{"time": "16:19:06.288", "level": "INFO", "logger": "L_Default", "text": "Some message"}
```

Multi-line messages use `\n` within the `text` field.
