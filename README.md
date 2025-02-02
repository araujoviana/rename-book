# rename-book

Command-line tool written in Rust that renames PDF and EPUB files to kebab-case.

It takes a single file path as an argument.

## Why

Because I needed a quick way to organize my documents and books 🐱.

## Usage

```bash
rename-book <file-path>
```

- `<file-path>`:  The path to the PDF or EPUB file you want to rename.

This tool performs multiple checks to verify and process files, specifically PDFs and EPUBs, renaming them with kebab-case stems while preserving the file extension.

## Dependencies

- `colored = "3.0"`
- `heck = "0.5"`

## Installation:

1.  Make sure you have [Rust and Cargo](https://rustup.rs/) installed.
2.  Clone the project and go into its directory

    ``` bash
    git clone https://github.com/araujoviana/rename-book
    cd rename-book
    ```

4.  Build and install the executable:

    ```bash
    cargo install --path .
    ```

After installation, you can run `rename-book` from anywhere in your terminal.

🚀
