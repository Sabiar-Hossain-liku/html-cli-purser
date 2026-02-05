## HTML Parser CLI Tool

This is a simple command-line HTML parser written in Rust. It allows you to parse HTML files or strings and extract useful information.

### Features
- Parse HTML from files or standard input
- Extract and display HTML elements
- Lightweight and fast

### Usage
Build the project with Cargo:

```sh
cargo build --release
```

Run the parser:

```sh
./target/release/htmlparser <input.html>
```

Replace `<input.html>` with your HTML file. You can also pipe HTML content directly.

### Requirements
- Rust (latest stable)

### License
MIT
