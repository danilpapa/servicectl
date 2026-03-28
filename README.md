# servicectl

A simple terminal tool for managing Docker Compose services with a TUI (Terminal User Interface).

`servicectl` helps you quickly select and run services from a `docker-compose.yml` file using an interactive interface

---

## Features

- Interactive TUI built with `Ratatui`
- Select multiple services using keyboard
- Run selected services via Docker Compose
- Automatically builds images before starting containers
- Works in the current project directory

---

## Requirements

- Rust
- Docker

---

## Installation

### From GitHub

```bash
cargo install --git https://github.com/danilpapa/servicectl