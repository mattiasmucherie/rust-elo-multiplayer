# rust-elo-multiplayer

[![](https://img.shields.io/crates/v/rust-elo-multiplayer)](https://crates.io/crates/rust-elo-multiplayer)

[![](https://img.shields.io/crates/d/rust-elo-multiplayer)](https://crates.io/crates/rust-elo-multiplayer)

Rust crate for calculating multiplayer rating based on elo ranking system

## Example usage

```rust
use rust_elo_multiplayer::EloRank;

fn main() {
    let players: Vec<f64> = vec![1000.0, 1000.0, 1000., 1000.0];
    let elo = EloRank { k: 32, players };
    let expected: Vec<f64> = elo.calculate();
}

```
