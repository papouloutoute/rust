# Notes de Formation Rust - Jour 1

## 1. Variables et Types

### Déclaration de variables
```rust
let nom = "Erwan";              // String literal
let _age: u32 = 23;             // Entier non signé 32 bits
let age_chien = 20;             // i32 par défaut
let temperature: f32 = 21.5;    // Float 32 bits
```

### Types numériques
- `i32` : signé (-2xxx à +2xxx)
- `u32` : non signé (0 à 4xxx)
- `i64` : signé très grand interval
- `u8` : non signé (0 à 255)
- `f32`, `f64` : nombres à virgule flottante

### Conventions
- **snake_case** obligatoire
- Pas de chiffre en début
- Pas d'espaces ni tirets

## 2. Fonctions

### Syntaxe de base
```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b  // Pas de point-virgule pour le retour
}

fn say_hello(nom: &str) {
    println!("Bonjour, {}", nom);
}
```

### Points clés
- `fn` pour définir une fonction
- `&str` = référence à une chaîne de caractères
- `->` pour spécifier le type de retour

## 3. Structures de Contrôle

### Conditions
```rust
if nombre % 2 == 0 {
    println!("{} est pair", nombre);
} else {
    println!("nombre impair");
}
```

### Boucles
```rust
// For avec intervalle inclusif
for i in 1..=10 {
    println!("i vaut {}", i);  // 1 à 10
}

// For avec intervalle exclusif
for j in 1..10 {
    println!("j vaut {}", j);  // 1 à 9
}

// Loop infinie
let mut compteur = 0;
loop {
    println!("Compteur: {}", compteur);
    compteur += 1;
    if compteur == 3 {
        break;
    }
}

// While
let mut compteur2 = 0;
while compteur2 < 4 {
    println!("Compteur2: {}", compteur2);
    compteur2 += 1;
}
```

## 4. Collections

### Tableaux statiques
```rust
let voitures = ["citroen", "renault", "jeep"];
println!("{}", voitures[1]);  // Accès par index

// Itération simple
for voiture in voitures {
    println!("voiture: {}", voiture);
}

// Itération avec index
for (i, voiture) in voitures.iter().enumerate() {
    println!("Index {}: {}", i, voiture);
}
```

### Vecteurs (tableaux dynamiques)
```rust
let noms = vec![
    String::from("Erwan"),
    String::from("Salma"),
    String::from("Pierre Emmanuel")
];

for (i, nom) in noms.iter().enumerate() {
    println!("Nom {}: {}", i, nom);
}
```

## 5. Entrées Utilisateur

### Lecture depuis stdin
```rust
use std::io;

let mut choix = String::new();
io::stdin().read_line(&mut choix).expect("Erreur de lecture");

let choix: usize = match choix.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Veuillez saisir un numéro valide");
        return;
    }
};
```

## 6. Structures (Struct)

### Définition et utilisation
```rust
struct Salarie {
    nom: String,
    prenom: String,
    age: u32
}

// Création d'instance
let salarie = Salarie {
    nom: String::from("Harbaoui"),
    prenom: String::from("Aymen"),
    age: 40
};

// Accès aux champs
println!("Nom: {}, Prénom: {}, Age: {}", 
         salarie.nom, salarie.prenom, salarie.age);
```

### Implémentation de méthodes
```rust
impl Salarie {
    fn afficher(&self) {
        println!("Salarié: {} {} ({})", 
                 self.nom, self.prenom, self.age);
    }
}

// Utilisation
salarie.afficher();
```

## 7. Exemple Pratique : CompteBancaire

### Structure complète avec méthodes
```rust
struct CompteBancaire {
    nom: String,
    solde: f64
}

impl CompteBancaire {
    // Lecture seule
    fn afficher(&self) {
        println!("Compte de {} : {} €", self.nom, self.solde);
    }
    
    // Modification (mut self)
    fn deposer(&mut self, montant: f64) {
        self.solde += montant;
        println!("+{} € déposés", montant);
    }
    
    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("-{} € retirés", montant);
        } else {
            println!("Solde insuffisant !");
        }
    }
    
    // Consommation (self)
    fn fermer(self) {
        println!("Compte de {} fermé. Solde final: {} €", 
                 self.nom, self.solde);
    }
}
```

### Utilisation
```rust
let mut compte = CompteBancaire {
    nom: String::from("Salma"),
    solde: 3000.0
};

compte.afficher();
compte.deposer(30.0);
compte.retirer(20.0);
compte.afficher();
compte.fermer();  // Consomme l'objet
```

## 8. Traits (Équivalent des Interfaces)

### Définition d'un trait
```rust
trait Animal {
    fn chanter(&self);
    fn crier(&self);
}

struct Chien;

impl Animal for Chien {
    fn chanter(&self) {
        println!("wouaf wouaf wouaf !!!");
    }
    
    fn crier(&self) {
        println!("WOUAF WOUAF !");
    }
}
```

## 9. Concepts Clés Rust

### Ownership (Propriété)
- `&self` : emprunt immutable (lecture seule)
- `&mut self` : emprunt mutable (modification)
- `self` : prise de propriété (consommation)

### Mutabilité
- Variables **immutables** par défaut
- `mut` pour rendre une variable modifiable
- `let mut variable = valeur;`

### Sécurité
- Pas d'erreurs de segmentation
- Gestion mémoire automatique
- Vérifications à la compilation

## 10. Compilation et Exécution

### Fichier simple
```bash
rustc intro.rs    # Compile
./intro          # Exécute
```

### Projet Cargo
```bash
cargo new mon_projet  # Nouveau projet
cargo run            # Compile et exécute
cargo build          # Compile seulement
```

## 11. Ressources

- Documentation officielle : https://doc.rust-lang.org/rust-by-example/
- Convention de nommage : snake_case
- Bibliothèque I/O : `use std::io;` 