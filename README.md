# Single Binary Web written in Rust
Build and Pack all frontend and backend files into one binary executable program

## Features
1. Using `yew` rust frontend framework
2. Using `actix-web` rust backend framework
3. Using `cargo-make` build tool

## Usage
1. Install wasm-pack and cargo-make

```bash
cargo install -f wasm-pack cargo-make
```

2. Init

```bash
# install tools
cargo make deps

# build project
cargo make build
```

3. Run

```bash
cargo run
```

and open `http://localhost:8000`, ping api is `http://localhost:8000/ping`
