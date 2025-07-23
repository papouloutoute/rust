# Notes de Formation Rust - Jour 2

## 1. `main.rs`

### But principal
Démonstration de la gestion de fichiers, de l'ownership, de l'emprunt et des structures en Rust.

### Utilisation de modules
- `mod ecrire;` : pour l'écriture de fichiers
- `mod lire;` : pour la lecture de fichiers

### Gestion de fichiers
- Création d'un fichier via la structure `Fichier` (voir `ecrire.rs`)
- Écriture et lecture du contenu du fichier

### Dates et heures
- Utilisation de la crate externe `chrono` pour afficher la date et l'heure actuelles au format français.

### Notions fondamentales Rust
- **Ownership** : chaque valeur a un propriétaire unique, la mémoire est libérée automatiquement à la fin du scope.
- **Transfert d'ownership** : passage d'une variable à une fonction (ex : `greetings(prenom)`).
- **Clone** : duplication d'une valeur pour garder l'originale.
- **Emprunt (`&`)** : permet de prêter une référence sans transfert d'ownership (`greetings2(&secu)`).
- **Membership** : appartenance à une structure (struct), exemple avec la struct `User`.

### Struct et méthodes
- Définition d'une struct `User` et affichage de ses champs via une fonction.
- Utilisation de méthodes associées à la struct `Fichier`.

---

## 2. `ecrire.rs`

### But
Encapsuler la logique d'écriture de fichiers dans une structure.

### Struct `Fichier`
- **Champs** :
  - `nom` (nom du fichier)
  - `contenu` (texte à écrire)
- **Méthode `creer`** : constructeur simple pour initialiser un fichier avec nom et contenu.
- **Méthode `ecrire`** : crée physiquement le fichier et écrit le contenu dedans.

### Utilisation de la bibliothèque standard
- `std::fs::File` pour la création de fichiers
- `std::io::Write` pour écrire dans le fichier

### Gestion d'erreurs
- Utilisation de `io::Result<()>` pour signaler les erreurs d'écriture.

---

## 3. `lire.rs`

### But
Ajouter la capacité de lire le contenu d'un fichier à la struct `Fichier`.

### Extension de la struct `Fichier`
- **Méthode `lire`** : ouvre le fichier, lit tout le contenu et l'affiche.

### Utilisation de la bibliothèque standard
- `std::fs::File` pour ouvrir le fichier
- `std::io::{BufReader, Read}` pour lire efficacement le contenu

### Gestion d'erreurs
- Utilisation de `io::Result<String>` pour signaler les erreurs de lecture et retourner le contenu lu.

---

## 4. Notions transversales

- **Modularité** : séparation claire entre l'écriture (`ecrire.rs`) et la lecture (`lire.rs`)
- **Gestion des erreurs** : usage systématique de `Result` pour la robustesse
- **Commentaires pédagogiques** : chaque fonction/méthode est commentée pour expliquer son rôle
- **Utilisation de crates externes** : `chrono` pour la gestion des dates

---

## 5. À retenir

- **Struct** : structure de données personnalisée (ex : `Fichier`, `User`)
- **Méthodes associées** : fonctions liées à une struct (ex : `ecrire`, `lire`)
- **Ownership & Borrowing** : gestion de la mémoire sûre et automatique
- **Gestion de fichiers** : création, écriture, lecture avec la stdlib Rust
- **Modularité** : code organisé en plusieurs fichiers pour la clarté

---

### Exemple de struct et méthodes
```rust
struct Fichier {
    nom: String,
    contenu: String,
}

impl Fichier {
    fn creer(nom: &str, contenu: &str) -> Self { /* ... */ }
    fn ecrire(&self) -> io::Result<()> { /* ... */ }
    fn lire(&self) -> io::Result<String> { /* ... */ }
}
```

### Exemple d'ownership et d'emprunt
```rust
let prenom = String::from("Julian");
let prenom2 = prenom.clone();
greetings(prenom); // ownership transféré
greetings2(&prenom2); // emprunt
```
