// TP1 - Système de gestion de comptes bancaires - VERSION CORRIGÉE
// Niveau débutant avec commentaires détaillés

use std::io::{self, Write}; // Bibliothèque pour les entrées/sorties (lecture clavier)

// Structure représentant un compte bancaire
#[derive(Clone)] // Permet de dupliquer un compte si nécessaire
struct CompteBancaire {
    nom: String,        // Nom du propriétaire du compte
    numero: u32,        // Numéro unique du compte
    solde: f64,         // Solde du compte (nombre décimal)
}

// Trait (interface) pour afficher les informations
trait Afficher {
    fn afficher(&self);
}

// Implémentation des méthodes pour CompteBancaire
impl CompteBancaire {
    // Constructeur - crée un nouveau compte bancaire
    fn nouveau(nom: String, numero: u32, solde_initial: f64) -> CompteBancaire {
        CompteBancaire {
            nom,
            numero,
            solde: solde_initial,
        }
    }

    // Méthode pour déposer de l'argent
    fn deposer(&mut self, montant: f64) {
        if montant > 0.0 {
            self.solde += montant;
            println!("Dépôt de {:.2} € effectué sur le compte de {}", montant, self.nom);
            println!("Nouveau solde : {:.2} €", self.solde);
        } else {
            println!("Le montant doit être positif !");
        }
    }

    // Méthode pour retirer de l'argent
    fn retirer(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Le montant doit être positif !");
            return;
        }
        
        if self.solde >= montant {
            self.solde -= montant;
            println!("Retrait de {:.2} € effectué du compte de {}", montant, self.nom);
            println!("Nouveau solde : {:.2} €", self.solde);
        } else {
            println!("Solde insuffisant ! Solde actuel : {:.2} €", self.solde);
        }
    }

    // Méthode pour obtenir le numéro du compte
    fn obtenir_numero(&self) -> u32 {
        self.numero
    }
}

// Implémentation du trait Afficher pour CompteBancaire
impl Afficher for CompteBancaire {
    fn afficher(&self) {
        println!("Compte n°{} - {} : {:.2} €", self.numero, self.nom, self.solde);
    }
}

// Fonction pour afficher le menu principal
fn afficher_menu() {
    println!("\n=== SYSTÈME DE GESTION BANCAIRE ===");
    println!("1. Créer un nouveau compte");
    println!("2. Afficher tous les comptes");
    println!("3. Déposer de l'argent");
    println!("4. Retirer de l'argent");
    println!("5. Fermer un compte");
    println!("6. Quitter l'application");
    println!("==========================================");
    print!("Votre choix (1-6) : ");
    io::stdout().flush().unwrap(); // Force l'affichage immédiat
}

// Fonction pour lire une ligne depuis le clavier
fn lire_entree() -> String {
    let mut input = String::new(); // Créer une nouvelle chaîne vide
    io::stdin()
        .read_line(&mut input) // Lire depuis le clavier
        .expect("Erreur lors de la lecture"); // Gérer les erreurs
    input.trim().to_string() // Supprimer les espaces et retourner
}

// Fonction pour lire un nombre entier
fn lire_nombre_entier() -> Result<u32, std::num::ParseIntError> {
    let input = lire_entree();
    input.parse::<u32>() // Convertir en nombre entier
}

// Fonction pour lire un nombre décimal
fn lire_nombre_decimal() -> Result<f64, std::num::ParseFloatError> {
    let input = lire_entree();
    input.parse::<f64>() // Convertir en nombre décimal
}

// Fonction pour créer un nouveau compte
fn creer_compte(comptes: &mut Vec<CompteBancaire>, prochain_numero: &mut u32) {
    println!("\n=== CRÉATION D'UN NOUVEAU COMPTE ===");
    println!("Suivez les étapes pour créer un nouveau compte bancaire.");
    println!();
    
    // Étape 1 : Demander le nom
    print!("Étape 1/2 - Nom du propriétaire : ");
    io::stdout().flush().unwrap(); // Force l'affichage
    let nom = lire_entree();
    
    if nom.is_empty() {
        println!("Le nom ne peut pas être vide !");
        return;
    }
    
    // Étape 2 : Demander le solde initial
    print!("Étape 2/2 - Solde initial (en €) : ");
    io::stdout().flush().unwrap(); // Force l'affichage
    match lire_nombre_decimal() {
        Ok(solde) => {
            if solde < 0.0 {
                println!("Le solde initial ne peut pas être négatif !");
                return;
            }
            
            let nouveau_compte = CompteBancaire::nouveau(nom, *prochain_numero, solde);
            println!();
            println!("✓ Compte créé avec succès !");
            nouveau_compte.afficher();
            
            comptes.push(nouveau_compte); // Ajouter le compte au vecteur
            *prochain_numero += 1; // Incrémenter le numéro pour le prochain compte
        }
        Err(_) => {
            println!("Veuillez entrer un montant valide !");
        }
    }
}

// Fonction pour afficher tous les comptes
fn afficher_comptes(comptes: &Vec<CompteBancaire>) {
    println!("\n=== LISTE DES COMPTES ===");
    
    if comptes.is_empty() {
        println!("Aucun compte n'existe actuellement.");
        return;
    }
    
    // Itérer sur tous les comptes et les afficher
    for compte in comptes {
        compte.afficher();
    }
    
    println!("Total des comptes : {}", comptes.len());
}

// Fonction pour trouver un compte par son numéro
fn trouver_compte_mut(comptes: &mut Vec<CompteBancaire>, numero: u32) -> Option<&mut CompteBancaire> {
    // Utiliser iter_mut() pour obtenir une référence mutable
    comptes.iter_mut().find(|compte| compte.obtenir_numero() == numero)
}

// Fonction pour déposer de l'argent
fn deposer_argent(comptes: &mut Vec<CompteBancaire>) {
    println!("\n=== DÉPÔT D'ARGENT ===");
    
    if comptes.is_empty() {
        println!("Aucun compte n'existe. Créez d'abord un compte !");
        return;
    }
    
    // Afficher la liste des comptes disponibles
    println!("Comptes disponibles :");
    for compte in comptes.iter() {
        compte.afficher();
    }
    println!();
    
    print!("Numéro du compte pour le dépôt : ");
    io::stdout().flush().unwrap();
    match lire_nombre_entier() {
        Ok(numero) => {
            match trouver_compte_mut(comptes, numero) {
                Some(compte) => {
                    print!("Montant à déposer (en €) : ");
                    io::stdout().flush().unwrap();
                    match lire_nombre_decimal() {
                        Ok(montant) => {
                            compte.deposer(montant);
                        }
                        Err(_) => {
                            println!("Veuillez entrer un montant valide !");
                        }
                    }
                }
                None => {
                    println!("Compte non trouvé ! Vérifiez le numéro.");
                }
            }
        }
        Err(_) => {
            println!("Veuillez entrer un numéro de compte valide !");
        }
    }
}

// Fonction pour retirer de l'argent
fn retirer_argent(comptes: &mut Vec<CompteBancaire>) {
    println!("\n=== RETRAIT D'ARGENT ===");
    
    if comptes.is_empty() {
        println!("Aucun compte n'existe. Créez d'abord un compte !");
        return;
    }
    
    // Afficher la liste des comptes disponibles
    println!("Comptes disponibles :");
    for compte in comptes.iter() {
        compte.afficher();
    }
    println!();
    
    print!("Numéro du compte pour le retrait : ");
    io::stdout().flush().unwrap();
    match lire_nombre_entier() {
        Ok(numero) => {
            match trouver_compte_mut(comptes, numero) {
                Some(compte) => {
                    print!("Montant à retirer (en €) : ");
                    io::stdout().flush().unwrap();
                    match lire_nombre_decimal() {
                        Ok(montant) => {
                            compte.retirer(montant);
                        }
                        Err(_) => {
                            println!("Veuillez entrer un montant valide !");
                        }
                    }
                }
                None => {
                    println!("Compte non trouvé ! Vérifiez le numéro.");
                }
            }
        }
        Err(_) => {
            println!("Veuillez entrer un numéro de compte valide !");
        }
    }
}

// Fonction pour fermer un compte
fn fermer_compte(comptes: &mut Vec<CompteBancaire>) {
    println!("\n=== FERMETURE DE COMPTE ===");
    
    if comptes.is_empty() {
        println!("Aucun compte n'existe !");
        return;
    }
    
    // Afficher la liste des comptes disponibles
    println!("Comptes disponibles :");
    for compte in comptes.iter() {
        compte.afficher();
    }
    println!();
    
    print!("Numéro du compte à fermer : ");
    io::stdout().flush().unwrap();
    match lire_nombre_entier() {
        Ok(numero) => {
            // Trouver l'index du compte à supprimer
            if let Some(index) = comptes.iter().position(|compte| compte.obtenir_numero() == numero) {
                let compte_ferme = comptes.remove(index); // Supprimer le compte du vecteur
                println!("Compte fermé :");
                compte_ferme.afficher();
                
                if compte_ferme.solde > 0.0 {
                    println!("Solde restant de {:.2} € remboursé au client", compte_ferme.solde);
                }
            } else {
                println!("Compte non trouvé ! Vérifiez le numéro.");
            }
        }
        Err(_) => {
            println!("Veuillez entrer un numéro de compte valide !");
        }
    }
}

// Fonction principale du programme
fn main() {
    println!("Bienvenue dans le système de gestion bancaire !");
    
    // Vecteur pour stocker tous les comptes bancaires
    let mut comptes: Vec<CompteBancaire> = Vec::new();
    
    // Variable pour suivre le prochain numéro de compte à attribuer
    let mut prochain_numero: u32 = 1;
    
    // Créer 3 comptes de démonstration
    println!("\nCréation de 3 comptes de démonstration...");
    comptes.push(CompteBancaire::nouveau("Alice Martin".to_string(), prochain_numero, 1500.0));
    prochain_numero += 1;
    comptes.push(CompteBancaire::nouveau("Bob Dupont".to_string(), prochain_numero, 2300.50));
    prochain_numero += 1;
    comptes.push(CompteBancaire::nouveau("Claire Bernard".to_string(), prochain_numero, 890.25));
    prochain_numero += 1;
    
    println!("Comptes de démonstration créés !");
    
    // Boucle principale du programme
    loop {
        afficher_menu();
        
        // Lire le choix de l'utilisateur
        match lire_nombre_entier() {
            Ok(choix) => {
                // Utiliser match pour traiter chaque choix
                match choix {
                    1 => {
                        creer_compte(&mut comptes, &mut prochain_numero);
                    }
                    2 => {
                        afficher_comptes(&comptes);
                    }
                    3 => {
                        deposer_argent(&mut comptes);
                    }
                    4 => {
                        retirer_argent(&mut comptes);
                    }
                    5 => {
                        fermer_compte(&mut comptes);
                    }
                    6 => {
                        println!("Merci d'avoir utilisé notre système bancaire !");
                        println!("Fermeture de l'application...");
                        break; // Sortir de la boucle principale
                    }
                    _ => {
                        println!("Choix invalide ! Veuillez choisir entre 1 et 6.");
                    }
                }
            }
            Err(_) => {
                println!("Veuillez entrer un numéro valide !");
            }
        }
        
        // Pause pour que l'utilisateur puisse lire les messages
        println!("\n--- Appuyez sur Entrée pour continuer ---");
        lire_entree();
    }
} 