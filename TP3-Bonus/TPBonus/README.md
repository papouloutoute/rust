# TP Bonus - Gestion Simplifiée de Fichiers en Rust

## Objectif
Créer une structure `Fichier` en Rust permettant de :
- Créer un fichier avec un nom, un contenu et une date de création
- Modifier et ajouter du contenu
- Vérifier l'existence du fichier
- Lire le contenu du fichier
- Utiliser des méthodes statiques et des getters

## Fonctionnalités principales
- **Struct Fichier** :
  - `nom` : nom du fichier
  - `contenu` : texte à écrire
  - `date_creation` : date/heure de création (avec chrono)
- **Méthodes** :
  - `new()` : constructeur
  - `creer_fichier()` : crée le fichier avec le contenu
  - `creer_avec_nom()` : méthode statique pour créer un fichier sans instance
  - `modifier_contenu()` : change tout le contenu
  - `ajouter_contenu()` : ajoute du texte à la fin
  - `existe()` : vérifie si le fichier existe
  - `get_nom()`, `get_contenu()`, `get_date_creation()` : getters

## Exemple d'utilisation
```rust
let mut mon_fichier = Fichier::new("demo.txt", "Ceci est le contenu initial.\n");
mon_fichier.creer_fichier().unwrap();
mon_fichier.ajouter_contenu("Ligne ajoutée.\n");
mon_fichier.modifier_contenu("Nouveau contenu complet.\n");
mon_fichier.creer_fichier().unwrap();
if mon_fichier.existe() {
    println!("Le fichier '{}' existe bien.", mon_fichier.get_nom());
}
Fichier::creer_avec_nom("autre.txt", "Fichier créé sans instance.\n").unwrap();
```

## Lancer le projet
```bash
cd TP3-Bonus/TPBonus
cargo run
```