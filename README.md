# Ryrite

A little curiosity, a few bytes, and a closer look inside your files. ✨

Ryrite is a small Rust command-line tool for static file inspection. It reads a file and prints its printable ASCII characters, a format hint, byte entropy, and—for ELF binaries—shared libraries and dynamic symbols. It inspects the file without executing it.

## What it does

- **Peek at the text:** displays printable ASCII bytes in their original order.
- **Recognize familiar formats:** checks signatures for ELF, PNG, JPEG, PDF, ZIP, and RIFF files.
- **Measure byte entropy:** prints a Shannon entropy estimate in bits per byte.
- **Explore ELF symbols:** lists shared libraries, imports, and exports from the dynamic symbol table.

The terminal output uses a little color to keep things easy to spot.

## Get started

You'll need Rust and Cargo with support for the Rust 2024 edition.

From the project directory, pass a file path to Cargo:

```bash
cargo run -- /path/to/binary
```

On Linux, you can try a small system binary:

```bash
cargo run -- /usr/bin/true
```

To build and run the release version:

```bash
cargo build --release
./target/release/ryrite /path/to/binary
```

Paths containing spaces should be wrapped in quotes.

## A few things to know

Ryrite is an early project, with a few rough edges:

- Format recognition covers more files than symbol analysis does. Symbol listing is implemented for ELF, and files that the binary parser cannot read may cause a panic—even if their format was recognized earlier.
- Each analysis pass reads the file in chunks of up to 8 MiB. Entropy is calculated per chunk, and format and symbol checks also run on each chunk, so larger files may produce incomplete results or parsing failures.
- Printable bytes are joined together; this is a simple text peek, without string boundaries or offsets.

For a first look, a small ELF binary is a good place to begin.

## Around the source

| File | Purpose |
| --- | --- |
| `src/main.rs` | Runs the analysis passes for the supplied path. |
| `src/file.rs` | Reads the file through a buffered, chunked reader. |
| `src/static_check.rs` | Handles printable text, format detection, entropy, and ELF symbols. |

Built with Rust, [Goblin](https://crates.io/crates/goblin) for binary parsing, and [colored](https://crates.io/crates/colored) for terminal colors.
