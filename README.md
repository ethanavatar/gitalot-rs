# gitalot-rs

A tool for interacting with more than one git repository at a time

## Usage

`example.txt`

```txt
https://github.com/ethanavatar/Results-py.git
https://github.com/ethanavatar/carpy.git
https://github.com/ethanavatar/getignore-rs.git
```

```bash
$ gitalot clone example.txt
...
```

## TODO

- [ ] fetch command
- [ ] pull command


## Installation

### From Source

```bash
$ git clone https://github.com/ethanavatar/gitalot-rs.git
$ cd getignore
$ cargo build --release
```

The binary is available at `target/release/gitalot.exe`