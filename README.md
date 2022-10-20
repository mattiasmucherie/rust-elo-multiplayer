# elo-multiplayer

[![](https://img.shields.io/crates/v/elo-multiplayer)](https://crates.io/crates/elo-multiplayer)

[![](https://img.shields.io/crates/d/elo-multiplayer)](https://crates.io/crates/elo-multiplayer)

Rust crate for calculating multiplayer rating based on elo ranking system

## Installation

If you are on Rust 1.62 or higher use `cargo add` to install the latest version:

```sh
cargo add elo-multiplayer
```

Alternatively, you can add the following to your `Cargo.toml` file manually:

```toml
[dependencies]
elo-multiplayer = "0.1.4"
```

## Example usage

```rust
use elo_multiplayer::EloRank;

fn main() {
    let players: Vec<f64> = vec![1000.0, 1000.0, 1000., 1000.0];
    let elo = EloRank { k: 32, players };
    let expected: Vec<f64> = elo.calculate();
}

```
