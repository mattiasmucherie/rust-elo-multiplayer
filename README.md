# elo-multiplayer

[![](https://img.shields.io/crates/v/elo-multiplayer)](https://crates.io/crates/elo-multiplayer)

[![](https://img.shields.io/crates/d/elo-multiplayer)](https://crates.io/crates/elo-multiplayer)

Rust crate for calculating multiplayer rating based on elo ranking system

## Mentions

This multiplayer elo algorithm is heavily inspired from [this](https://towardsdatascience.com/developing-a-generalized-elo-rating-system-for-multiplayer-games-b9b495e87802) article.

It works quite similar to the normal elo ranking.
But the expected scores calculated by matching all the players against each other.

$$
E_{A} = \sum_{1<i<N, i\neq A}
\left( \sum_{k=1}^n a_k^2 \right)
\left( \sum_{k=1}^n b_k^2 \right)
$$

<!-- E_A = {\sum_{1<i<N, i\neq A}{1\over 1+10^{(R_{i}-R_{A})/D}}\over {N(N-1)/2}} -->

where we have player $A$, number of players $N$ and the ranking of player $A$ is $R_A$.

And the score is a bit more complex then the usual two player elo rating.
Currently only a linear method of getting the scores for the players has been implemented:

$$
S_A^{linear} = {N-p_A\over N(N-1)/2}
$$

where $p_A$ is the postion of the player (1st, 2nd, etc...)

We can then calculated the new ranking of a player with:

$$
R'_{A} = R_{A} + K(N-1)(S_{A}-E_{A})
$$

Where $K$ is the usual [K-factor](https://en.wikipedia.org/wiki/Elo_rating_system#Most_accurate_K-factor).

## Installation

If you are on Rust 1.62 or higher use `cargo add` to install the latest version:

```sh
cargo add elo-multiplayer
```

Alternatively, you can add the following to your `Cargo.toml` file manually:

```toml
[dependencies]
elo-multiplayer = "0.1.5"
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
