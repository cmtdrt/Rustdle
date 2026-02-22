# Rustdle

Rustdle is a Rust adaptation of the Wordle game.
The game runs entirely in the terminal.

A secret word is drawn at random from a list of 8,059 five-letter French words (stored in `words.json`).

<img width="430" height="376" alt="image" src="https://github.com/user-attachments/assets/474f32e3-a04a-4ce9-9b47-0b35149e7fad" />


## Installation and running

Clone the repository, then run the game from the project root:

```bash
cargo run
```

By default, the game allows 6 attempts. You can change this by passing an argument:

```bash
cargo run -- 10    # 10 attempts
cargo run -- 3     # 3 attempts
```

## In-game
Quit the game by pressing `q` or `ctrl+c`.
At the end of a round, you can play again or not by pressing `o` or `n`.
