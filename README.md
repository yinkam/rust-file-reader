# Rust File Reader
#### This project is part of [Pragmatic AI Labs Rust Bootcamp](https://ds500.paiml.com/bootcamps/rust).

A command-line file reading utility built in Rust that demonstrates file I/O operations, error handling, and command-line argument processing.

## Overview

This project is a simple yet practical file reader application that reads text files line by line and outputs their contents to the terminal. It showcases fundamental Rust concepts including:

- File system operations
- Error handling with `Result` types
- Command-line argument processing
- Buffered I/O for efficient file reading

## Features

Currently implemented:
- ✅ Basic file reading functionality
- ✅ Command-line argument processing
- ✅ Basic error handling for file not found and I/O errors
- ✅ Line-by-line file processing with buffered reading

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yinkam/rust-file-reader.git
   cd rust-file-reader
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. For optimized release build:
   ```bash
   cargo build --release
   ```

## Usage

### Basic Usage
```bash
# Run with cargo (development)
cargo run -- path/to/your/file.txt

# Or use the built binary (debug)
./target/debug/rust-file-reader path/to/your/file.txt

# Or use the optimized binary (release)
./target/release/rust-file-reader path/to/your/file.txt
```

### Examples
```bash
# Read a text file
cargo run -- README.md

# Read a configuration file
cargo run -- Cargo.toml

# Read any text-based file
cargo run -- src/main.rs
```

## Feature Roadmap

### Beginner Features
- ✅ Basic file reading
- ✅ Command-line argument handling
- ✅ Basic error handling
- [ ] Help message (`--help` flag)
- [ ] Version information (`--version` flag)
- [ ] Read from stdin when no file specified
- [ ] Basic logging with different levels
- [ ] Unit tests for core functionality
- [ ] Integration tests
- [ ] Documentation comments

### Intermediate Features
- [ ] Multiple file support
- [ ] Line numbering option (`--line-numbers`)
- [ ] Character and word counting
- [ ] File encoding detection and handling
- [ ] Colored output for syntax highlighting
- [ ] Paging support for large files
- [ ] Search functionality within files
- [ ] Configuration file support
- [ ] Benchmarking and performance tests
- [ ] Cross-platform compatibility testing

### Advanced Features
- [ ] Async file reading for large files
- [ ] Memory-mapped file reading
- [ ] Compressed file support (gzip, zip)
- [ ] Binary file hex dump mode
- [ ] Tail-like functionality (follow file changes)
- [ ] Regular expression filtering
- [ ] Plugin system for custom file processors
- [ ] REST API for file reading service
- [ ] WebAssembly compilation target
- [ ] Fuzzing tests for robustness

## Architecture

The application follows a simple command-line tool architecture:

- **main.rs**: Entry point handling argument parsing and orchestrating file operations
- **File I/O**: Uses `std::fs::File` with `BufReader` for efficient line-by-line reading
- **Error Handling**: Pattern matching on `Result` types for graceful error management

## Testing

Run the test suite with:
```bash
cargo test
```

For verbose output:
```bash
cargo test -- --nocapture
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

[Add your chosen license]

---

*Built during the [Pragmatic AI Labs Rust Bootcamp](https://github.com/paiml/ds500-rust-bootcamp)*
