# grep-rs 🦀

## Description
A simple grep clone written in Rust.
A great way to learn Rust or any other programming language, is by writing CLI tools
such as Unix utilities like grep, find, cat, ls, ... etc.
Writing such tools allow you to explore the languages standard library,
performing basic tasks like Reading/Writing Files, Parsing and Manipulating Strings,
working with Data Structures, Error Handling, Debugging, and so forth.
Refrain from using third party crates as building things from scratch will help you
learn the language in a deeper level.

## Installation

1. Clone the repository
```bash
git clone https://github.com/yourusername/grep-rs.git
```
2. Navigate to the repository
```bash
cd grep-rs
```
3. Build the project
```bash
cargo build --release
```
4. The binary will be located in `target/release/greprs`

## Usage

```bash
./greprs [PATTERN] [FILE]
```