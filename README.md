# Local CLI Pastebin

A lightweight, file-based CLI pastebin written in **Rust**.  
Create, read, list, and delete text snippets locally with zero setup — no database, no server, no cloud.

## Features

- **Create** — Save text snippets with a random 6-character alphanumeric ID.
- **Read** — Retrieve pastes instantly by ID.
- **List** — View all stored paste IDs at a glance.
- **Delete** — Clean up pastes you no longer need.
- **Persistent** — All data stored locally in a hidden `.pastes/` directory.
- **Safe** — Graceful error handling; no panics on missing files.

---

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Build from source
```bash
git clone https://github.com/yourusername/local-cli-pastebin.git
cd local-cli-pastebin
cargo build --release
```
### Usage
```bash
# Create a new paste
pastebin new "Here is my secret snippet of code!"
```
```
# Read a paste by ID
pastebin read Xy7b9P
```
```
# List all stored pastes
pastebin list
```
```
# Delete a paste
pastebin delete Xy7b9P
```
