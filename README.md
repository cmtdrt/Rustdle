# Rustdle

Rustdle est une adaptation du jeu Wordle écrite en Rust.
Le jeu se joue entièrement dans le terminal.

Un mot secret est tiré aléatoirement parmi la liste des 8 059 mots de 5 lettres de la langue française. (liste située dans `words.json`).

<img width="430" height="376" alt="image" src="https://github.com/user-attachments/assets/474f32e3-a04a-4ce9-9b47-0b35149e7fad" />


## Installation et lancement

Clonez le dépôt, puis lancez le jeu depuis la racine du projet :

```bash
cargo run
```

Par défaut, le jeu permet 6 tentatives. Vous pouvez personnaliser ce nombre en passant un argument :

```bash
cargo run -- 10    # 10 tentatives
cargo run -- 3     # 3 tentatives
```

# En jeu
Quittez le jeu en appuyant sur `q` ou `ctrl+c`.
A la fin d'une partie, vous pouvez relancer le jeu ou non en appuyant sur `o` ou `n`.
