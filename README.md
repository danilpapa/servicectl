# servicectl

A simple terminal tool for managing Docker Compose services with a TUI (Terminal User Interface).

`servicectl` helps you quickly select and run services from a `docker-compose.yml` file using an interactive interface

---

## Features

- Interactive TUI built with `Ratatui`
- Select multiple services using keyboard
- Choose build options before running selected services
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
```

---

## Usage

Run `servicectl` inside a project directory that contains `docker-compose.yml`.

1. Select one or more services in the first screen.
2. Press `Enter` to open the options screen.
3. Choose one of the available actions:
   `Build only selected services` or `Build all services`.
4. Press `Enter` to run the selected option.
5. Press `Esc` to return to the previous screen.
