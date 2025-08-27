# Arca

**Arca** is a web application built with Rust using the high-performance [`actix-web`](https://actix.rs/) framework. It provides a robust backend API and serves a static frontend, making it suitable as a foundation for modern web projects.

## Features

- **RESTful API** built with `actix-web`
- **Static file serving** for frontend assets (HTML, CSS, JS)
- Modular route organization (e.g., `/echo`, `/deploy`)
- Configurable via TOML file (`config.example.toml`)
- Error handling and middleware support
- Ready-to-use project structure for rapid development

## Technologies Used

- **Rust** (backend language)
- **[actix-web](https://actix.rs/)** (web framework)
- **HTML, CSS, JavaScript** (frontend, in `public/`)
- **TOML** (configuration)

## Getting Started

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (stable)
- (Optional) [cargo-watch](https://crates.io/crates/cargo-watch) for live reload during development

### Installation

1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd arca
   ```

2. **Copy and edit the configuration file:**
   ```sh
   cp config.example.toml config.toml
   # Edit config.toml as needed
   ```

3. **Build the project:**
   ```sh
   cargo build --release
   ```

### Running the Application

Start the server with:

```sh
cargo run
```

By default, the backend will start and serve the frontend from the `public/` directory. Open your browser and navigate to the address shown in the terminal (commonly `http://127.0.0.1:8000`).

### Usage

- **API Endpoints:**  
  The backend exposes endpoints such as `/echo` and `/deploy`.  
  Example (using `curl`):
  ```sh
  curl -X POST http://127.0.0.1:8000/echo -d '{"message":"Hello"}' -H "Content-Type: application/json"
  ```

- **Frontend:**  
  Access the static frontend at the root URL (e.g., `http://127.0.0.1:8000/`).

### Configuration

- Edit `config.toml` to adjust server settings, ports, or other options as needed.

## Project Structure

```
arca/
├── Cargo.toml
├── config.example.toml
├── public/
│   ├── index.html
│   ├── main.js
│   └── style.css
└── src/
    ├── main.rs
    ├── lib.rs
    ├── config.rs
    ├── error.rs
    ├── state.rs
    ├── cli.rs
    └── routes/
        ├── mod.rs
        ├── echo.rs
        ├── deploy.rs
        └── middleware.rs
```

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Commit your changes with clear messages.
4. Open a pull request describing your changes.

Please ensure your code follows Rust best practices and includes relevant tests.

## License

This project is licensed under the MIT License.