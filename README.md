# SBM - Simple Bookmark ![Rust](https://github.com/malcjones/sbm/actions/workflows/rust.yml/badge.svg)

This repository contains the rough living spec for the SBM file format, along with a reference implementation in Rust.
This implementation is well-covered by tests, and is designed to be easy to use and extend. It implements a parser and an encoder for the SBM file format. The encoder is implemented as the `Display` trait, so it can be used with the `write!` and `format!` macros.

## rough spec
SBM is a file format for storing and categorizing bookmarks. It is designed to be simple and easy to use. It is also designed to be easy to parse and manipulate with a computer program.

## format
The main concept in SBM is pipe-delimited lines. Each
entry (category header, bookmark, or comment) is on its own line. The first character of the line determines the type of entry.

### Categories
A category consists of a header, and a list of bookmarks. The header is a line that starts with a `#` character, followed by the category name & an optional icon. Lines after the header are bookmarks. A category ends when another category header is encountered.

Examples:

with icon:
```
# Category Name | icon
```
without icon:
```
# Category Name
```

### Bookmarks
A bookmark is a line that starts with anything other than `#` or `//`. The first field is the name of the bookmark, followed by the description, and the URL. The fields are separated by a `|` character.

Example:
```
Bookmark Name | Description | https://example.com
```

### Comments
A comment is a line that starts with `//`. It is ignored by the parser. Empty lines are also ignored.

Example:
```
// This is a comment
```