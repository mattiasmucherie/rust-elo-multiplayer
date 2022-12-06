# elo-multiplayer

[![](https://img.shields.io/crates/v/elo-multiplayer)](https://crates.io/crates/elo-multiplayer)

[![](https://img.shields.io/crates/d/elo-multiplayer)](https://crates.io/crates/elo-multiplayer)

Rust crate for calculating multiplayer rating based on elo ranking system

## Mentions

This multiplayer elo algorithm is implementation in rust of the equations mentioned in [this](https://towardsdatascience.com/developing-a-generalized-elo-rating-system-for-multiplayer-games-b9b495e87802) article.

It works quite similar to the normal elo ranking.
But the expected scores calculated by matching all the players against each other.

$$
E_A = {\sum_{i=1, i\neq A}^{N}{1\over 1+10^{(R_{i}-R_{A})/D}}\over {N(N-1)/2}}
$$

where we have player $A$, number of players $N$ and the ranking of player $A$ is $R_A$.

And the score is a bit more complex then the usual two player elo rating.

We can choose between two different scoring functions. A $linear$ and an $exponential$ one.
The $linear$ scoring funtion makes it even for everybody, while the $exponential$ one favors the winner a bit more, making it more competetive and more important to score high.

$$
S_A^{linear}(p_A) = {N-p_A\over N(N-1)/2}
$$

$$
S_A^{exp}(p_A,\alpha) = {\alpha^{N-p_A}-1 \over \sum_{i=1}^N (\alpha^{N-i}-1)} ; \alpha \in (1,\infty)
$$

where $p_A$ is the postion of the player (1st, 2nd, etc...) and $\alpha$ is the base of the exponential function.

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
elo-multiplayer = "0.2.4"
```

## Example usage

```rust
use elo_multiplayer::EloRank;

fn main() {
    let players: Vec<f64> = vec![1000.0, 1000.0, 1000.0, 1000.0];
    let elo = EloRank { players, ..Default::default() };
    let new_rankings: Vec<f64> = elo.calculate();
}

```
