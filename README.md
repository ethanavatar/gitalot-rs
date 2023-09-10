# gitalot-rs

A tool for interacting with more than one git repository at a time. This program was made for practice purposes and was not intended to be used. Feel free to use the idea for your own project

## Usage

`example.txt`

```txt
https://github.com/ethanavatar/Results-py.git
https://github.com/ethanavatar/carpy.git
https://github.com/ethanavatar/getignore-rs.git
```

```bash
$ gitalot clone example.txt dest/
```

## Installation

After installing Rust via [rustup](https://rustup.rs/). You can install using cargo:
```bash
$ cargo install gitalot --git https://github.com/ethanavatar/gitalot-rs.git
```
