# Local CLI Pastebin

A lightweight, file-based CLI pastebin written in **Rust**.  
Create, read, list, and delete text snippets locally with zero setup — no database, no server, no cloud.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)

---

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