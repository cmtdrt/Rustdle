use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

const DEFAULT_MAX_TRIES: usize = 6;

fn main() {
    // Nombre d'essais maximum
    let max_tries = parse_max_tries().unwrap_or(DEFAULT_MAX_TRIES);

    // Chargement des mots depuis le fichier words.json
    let words = match load_words("words.json") {
        Ok(w) if !w.is_empty() => w,
        Ok(_) => {
            eprintln!("Le fichier words.json est vide");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Impossible de charger words.json: {e}");
            std::process::exit(1);
        }
    };

    let mut rng = thread_rng();

    loop {
        // Tirage du mot secret
        let secret = words.choose(&mut rng).expect("liste de mots non vide").to_string();

        let word_set: std::collections::HashSet<&str> = words.iter().map(|s| s.as_str()).collect();

        println!("Rustdle — devine le mot de 5 lettres ({} essais).", max_tries);
        println!("Indices: vert = bien placé, jaune = présent, gris = absent.\n");

        let mut history: Vec<String> = Vec::new();
        let mut tries = 0usize;
        let mut won = false;

        while tries < max_tries {
            for line in &history {
                println!("{line}");
            }

            let guess = match prompt_guess(&word_set) {
                Some(g) => g,
                None => {
                    println!("\nBye.");
                    return;
                }
            };

            let rendered = render_guess(&secret, &guess);
            history.push(rendered);
            tries += 1;

            if guess == secret {
                won = true;
                for line in &history {
                    println!("{line}");
                }
                println!("\nBravo ! Le mot était: {}", secret.to_uppercase());
                break;
            }

            println!();
        }

        if !won {
            for line in &history {
                println!("{line}");
            }
            println!("\nDommage… Le mot était: {}", secret.to_uppercase());
        }

        if !prompt_replay() {
            break;
        }

        println!();
    }
}

// Parser le nombre d'essais maximum
fn parse_max_tries() -> Option<usize> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        args[1].parse().ok()
    } else {
        None
    }
}

// Charger les mots depuis un fichier
fn load_words(path: &str) -> Result<Vec<String>, String> {
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let raw: Vec<String> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(raw.len());
    for w in raw {
        let w = w.trim().to_lowercase();
        if w.len() == 5 && w.chars().all(|c| c.is_ascii_lowercase()) {
            out.push(w);
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

// Demander une proposition de mot
fn prompt_guess(word_set: &std::collections::HashSet<&str>) -> Option<String> {
    loop {
        println!();
        print!("Votre proposition : ");
        let _ = io::stdout().flush();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            println!("\nErreur de lecture.");
            continue;
        }
        let s = buf.trim().to_lowercase();
        if s == "q" || s == "quit" || s == "exit" {
            return None;
        }
        if s.len() != 5 || !s.chars().all(|c| c.is_ascii_lowercase()) {
            println!("Entrée invalide: 5 lettres a-z uniquement.");
            continue;
        }
        if !word_set.contains(s.as_str()) {
            println!("Mot inconnu (pas dans words.json).");
            continue;
        }
        return Some(s);
    }
}

// Enum pour les marqueurs de couleur
#[derive(Copy, Clone, PartialEq, Eq)]
enum Mark {
    Gray, // Lettre non présente
    Yellow, // Lettre présente mais mal placée
    Green, // Lettre présente et bien placée
}

// Afficher une proposition de mot
fn render_guess(secret: &str, guess: &str) -> String {
    let s: Vec<char> = secret.chars().collect();
    let g: Vec<char> = guess.chars().collect();

    let mut marks = vec![Mark::Gray; 5];

    // 1) Marque les verts + compte ce qui reste dans le secret
    let mut remaining: HashMap<char, usize> = HashMap::new();
    for i in 0..5 {
        if g[i] == s[i] {
            marks[i] = Mark::Green;
        } else {
            *remaining.entry(s[i]).or_insert(0) += 1;
        }
    }

    // 2) Marque les jaunes en fonction du stock restant
    for i in 0..5 {
        if marks[i] == Mark::Green {
            continue;
        }
        if let Some(n) = remaining.get_mut(&g[i]) {
            if *n > 0 {
                marks[i] = Mark::Yellow;
                *n -= 1;
            }
        }
    }

    // 3) Rend avec couleurs ANSI
    let mut out = String::new();
    for i in 0..5 {
        let c = g[i].to_ascii_uppercase();
        let chunk = match marks[i] {
            Mark::Green => format!("\x1b[30;42m {c} \x1b[0m"),
            Mark::Yellow => format!("\x1b[30;43m {c} \x1b[0m"),
            Mark::Gray => format!("\x1b[30;100m {c} \x1b[0m"),
        };
        out.push_str(&chunk);
    }
    out
}

// Demander si le joueur veut rejouer
fn prompt_replay() -> bool {
    loop {
        print!("\nRejouer ? (o/n): ");
        let _ = io::stdout().flush();
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            return false;
        }
        match buf.trim().to_lowercase().as_str() {
            "o" | "oui" | "y" | "yes" => return true,
            "n" | "non" | "no" => return false,
            _ => println!("Réponds par o(oui) ou n(non)."),
        }
    }
}


