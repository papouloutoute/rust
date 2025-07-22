// Exemple pratique : Gestion d'un compte bancaire en Rust

struct CompteBancaire {
    nom: String,
    solde: f64  // Correction: f64 au lieu de 100
}

impl CompteBancaire {
    // Lecture seule - afficher les informations du compte
    fn afficher(&self) {
        println!("Compte bancaire de {} : {} €", self.nom, self.solde);
    }

    // Modification - déposer de l'argent
    fn deposer(&mut self, montant: f64) {
        self.solde += montant;
        println!("+{} € déposés", montant);  // Correction: println! au lieu de println
    }

    // Modification - retirer de l'argent
    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {  // Correction: >= au lieu de > =
            self.solde -= montant;  // Correction: self.solde au lieu de self.montant
            println!("-{} € retirés", montant);
        } else {
            println!("Solde insuffisant !");
        }
    }

    // Consommation - fermer le compte
    fn fermer(self) {
        println!("Le compte de {} est fermé, dernier solde : {} €", self.nom, self.solde);
    }
}

fn main() {
    let mut compte = CompteBancaire {  // Correction: CompteBancaire au lieu de compteBancaire
        nom: String::from("Salma"),    // Correction: , au lieu de ;
        solde: 3000.0
    };

    compte.afficher();
    compte.deposer(30.0);
    compte.retirer(20.0);
    compte.afficher();

    compte.fermer();
    // Après fermer(), compte n'est plus accessible car il a été consommé
}

// Exemple de Traits - Équivalent des interfaces en Rust
trait Animal {
    fn chanter(&self);
    fn crier(&self);  // Correction: &self au lieu de $self
}

struct Chien;

impl Animal for Chien {
    fn chanter(&self) {
        println!("wouaf wouaf wouaf !!!");
    }
    
    fn crier(&self) {
        println!("WOUAF WOUAF TRÈS FORT !!!");
    }
}

struct Oiseau;

impl Animal for Oiseau {
    fn chanter(&self) {
        println!("cui cui cui ♪♫");
    }
    
    fn crier(&self) {
        println!("CRIII CRIII !!!");
    }
}

// Fonction qui utilise le trait Animal
fn faire_du_bruit(animal: &dyn Animal) {
    animal.chanter();
    animal.crier();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compte_bancaire() {
        let mut compte = CompteBancaire {
            nom: String::from("Test"),
            solde: 1000.0
        };
        
        compte.deposer(100.0);
        assert_eq!(compte.solde, 1100.0);
        
        compte.retirer(50.0);
        assert_eq!(compte.solde, 1050.0);
    }
    
    #[test]
    fn test_traits() {
        let chien = Chien;
        let oiseau = Oiseau;
        
        faire_du_bruit(&chien);
        faire_du_bruit(&oiseau);
    }
} 