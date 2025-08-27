# Arca

**Arca** is a web application built with Rust using the high-performance [`Axum`](https://docs.rs/axum/) framework. It provides a robust backend API and serves a static frontend, making it suitable as a foundation for modern web projects.

## Features

- **RESTful API** built with `Axum`
- **Static file serving** for frontend assets (HTML, CSS, JS)
- Modular route organization (e.g., `/echo`, `/deploy`)
- Configurable via TOML file (`config.example.toml`)
- Error handling and middleware support
- Ready-to-use project structure for rapid development

## Technologies Used

- **Rust** (backend language)
- **[Axum](https://docs.rs/axum/)** (web framework)
- **[Tokio](https://tokio.rs/)** (asynchronous runtime)
- **[Serde](https://serde.rs/)** and **[serde_json](https://docs.rs/serde_json/)** (serialization/deserialization)
- **[Tower](https://docs.rs/tower/)** and **[tower-http](https://docs.rs/tower-http/)** (middleware and HTTP utilities)
- **[Clap](https://docs.rs/clap/)** (command-line argument parsing)
- **[Utoipa](https://docs.rs/utoipa/)** and **[utoipa-swagger-ui](https://docs.rs/utoipa-swagger-ui/)** (OpenAPI documentation and Swagger UI)
- **HTML, CSS, JavaScript** (frontend, in `public/`)
- **TOML** (configuration)

## Getting Started

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (stable)
- (Optional) [cargo-watch](https://crates.io/crates/cargo-watch) for live reload during development

### Installation and Setup Instructions

1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd arca
   ```

2. **Copy and edit the configuration file:**
   ```sh
   cp config.example.toml config.toml
   # Edit config.toml as needed (e.g., port, static file path, etc.)
   ```

3. **Build the project:**
   ```sh
   cargo build --release
   ```

### How to Run the Application

Start the server with:

```sh
cargo run
```

By default, the backend will start and serve the frontend from the `public/` directory. The default address is `http://127.0.0.1:8000` (unless changed in `config.toml`). Open your browser and navigate to this address.

#### Using cargo-watch (optional, for development):

```sh
cargo watch -x run
```

### Basic Usage Instructions

- **API Endpoints:**  
  The backend exposes endpoints such as `/echo` and `/deploy`.  
  Example (using `curl`):

  ```sh
  curl -X POST http://127.0.0.1:8000/echo -d '{"message":"Hello"}' -H "Content-Type: application/json"
  ```

  This will return a JSON response echoing your message.

- **Frontend:**  
  Access the static frontend at the root URL (e.g., `http://127.0.0.1:8000/`).

- **OpenAPI Documentation:**  
  If enabled, interactive API documentation is available at `/swagger-ui` (powered by utoipa and utoipa-swagger-ui).

### Configuration

- Edit `config.toml` to adjust server settings, such as port, static file directory, and other options.

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