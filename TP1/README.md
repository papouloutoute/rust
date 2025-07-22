# TP1 - Système de Gestion Bancaire

## Description
Système de gestion de comptes bancaires en Rust avec interface console.

## Fonctionnalités
- Créer un compte bancaire
- Afficher tous les comptes 
- Déposer/retirer de l'argent
- Fermer un compte
- Menu interactif

## Concepts Rust
- `struct` et `impl`
- `Vec<CompteBancaire>`
- `trait Afficher`
- Boucles `loop` et `match`
- Lecture console `std::io`
- Gestion d'erreurs `Result<T, E>`

## Utilisation

### Compilation
```bash
rustc main.rs
./main
```

### Avec Cargo
```bash
cargo run
```

### Tests
```bash
rustc tests.rs && ./tests
```

## Structure du Code
```rust
struct CompteBancaire {
    nom: String,
    numero: u32,
    solde: f64,
}
