# rsgrep

`rsgrep` is a simple command-line tool, similar to `grep`, implemented in Rust. It allows you to search for a specific pattern within a text file and prints the lines that contain the pattern.

## Purpose

This project was developed as a learning exercise to understand some of the core concepts of the Rust programming language, such as:

-   Reading and parsing command-line arguments.
-   File I/O operations.
-   Error handling with `Result` and `Option`.
-   Writing unit tests.
-   Basic string manipulation.
-   Organizing code into modules.

## Features

-   Search for a given pattern in a file.
-   Case-sensitive search by default.
-   Case-insensitive search using the `-i` flag or by setting the `IGNORE_CASE` environment variable.

## How to Run

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Steps

1.  **Clone the repository:**
    ```sh
    git clone <repository-url>
    cd rsgrep
    ```

2.  **Build the project:**
    ```sh
    cargo build --release
    ```

3.  **Run the program:**
    The basic command structure is:
    ```sh
    cargo run <pattern> <filename>
    ```

    **Example using the prepared `file.txt`:**

    The following command will search for the pattern "bog" in the `file.txt` file. The output will be redirected to `out.txt`, and any errors will be redirected to `error.txt`.

    ```sh
    cargo run bog file.txt > out.txt 2> error.txt
    ```

    **Case-Insensitive Search:**

    To perform a case-insensitive search, you can use the `-i` flag:
    ```sh
    cargo run -i Bog file.txt
    ```
    Alternatively, you can set the `IGNORE_CASE` environment variable:
    ```sh
    IGNORE_CASE=true cargo run bog file.txt
    ```

## Code Structure

-   `src/main.rs`: Handles command-line argument parsing and calls the main logic.
-   `src/lib.rs`: Contains the core logic of the application, including argument parsing, file reading, and the search functions. It also contains the unit tests.

## Future Improvements

-   Add support for regular expressions.
-   Implement more `grep` options, such as `-v` (invert match), `-c` (count lines), and `-n` (line number).
-   Add support for searching in multiple files or directories.
-   Improve performance for large files.
