# About

Do a quick check for outdated `[dependencies]` in `Cargo.toml` file.

# Installation

```
git clone https://github.com/euvoor/cargout
cd cargout
cargo build --release
```

# Usage

```
cargout 0.1.0
Oussama Elgoumri <euvoor@gmail.com>
List outdated crates

USAGE:
    cargout [FLAGS] [OPTIONS]

FLAGS:
    -a, --all        Show up to date dependencies as well
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    Path to Cargo.toml file
```

# Examples

```
$ cd /data/projects/websocket-rs
$ cargout
tracing-subscriber   0.2.8      0.2.12
tracing              0.1.16     0.1.21
```

# Todo
- [ ] Add support for workspaces
