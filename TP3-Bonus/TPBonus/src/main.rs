use chrono::{DateTime, Local};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;

// Structure Fichier : nom, contenu, date de création
pub struct Fichier {
    nom: String,
    contenu: String,
    date_creation: DateTime<Local>,
}

impl Fichier {
    // Constructeur new()
    pub fn new(nom: &str, contenu: &str) -> Self {
        Fichier {
            nom: nom.to_string(),
            contenu: contenu.to_string(),
            date_creation: Local::now(),
        }
    }

    // Méthode pour créer le fichier avec le contenu
    pub fn creer_fichier(&self) -> io::Result<()> {
        let mut fichier = File::create(&self.nom)?;
        fichier.write_all(self.contenu.as_bytes())?;
        println!("Fichier '{}' créé avec succès !", self.nom);
        Ok(())
    }

    // Méthode statique : créer un fichier directement
    pub fn creer_avec_nom(nom: &str, contenu: &str) -> io::Result<()> {
        let mut fichier = File::create(nom)?;
        fichier.write_all(contenu.as_bytes())?;
        println!("Fichier '{}' créé directement !", nom);
        Ok(())
    }

    // Modifier le contenu
    pub fn modifier_contenu(&mut self, nouveau: &str) {
        self.contenu = nouveau.to_string();
        println!("Contenu modifié.");
    }

    // Ajouter du contenu
    pub fn ajouter_contenu(&mut self, ajout: &str) {
        self.contenu.push_str(ajout);
        println!("Contenu ajouté.");
    }

    // Vérifier si le fichier existe
    pub fn existe(&self) -> bool {
        Path::new(&self.nom).exists()
    }

    // Getter pour le nom
    pub fn get_nom(&self) -> &str {
        &self.nom
    }
    // Getter pour le contenu
    pub fn get_contenu(&self) -> &str {
        &self.contenu
    }
    // Getter pour la date de création
    pub fn get_date_creation(&self) -> DateTime<Local> {
        self.date_creation
    }
}

fn main() {
    // Création d'une nouvelle instance
    let mut mon_fichier = Fichier::new("demo.txt", "Ceci est le contenu initial.\n");
    println!("Nom: {}", mon_fichier.get_nom());
    println!("Date de création: {}", mon_fichier.get_date_creation().format("%d/%m/%Y %H:%M:%S"));

    // Créer le fichier sur le disque
    mon_fichier.creer_fichier().unwrap();

    // Ajouter du contenu
    mon_fichier.ajouter_contenu("Ligne ajoutée.\n");
    // Modifier le contenu
    mon_fichier.modifier_contenu("Nouveau contenu complet.\n");
    // Re-créer le fichier avec le nouveau contenu
    mon_fichier.creer_fichier().unwrap();

    // Vérifier si le fichier existe
    if mon_fichier.existe() {
        println!("Le fichier '{}' existe bien.", mon_fichier.get_nom());
    }

    // Utiliser la méthode statique
    Fichier::creer_avec_nom("autre.txt", "Fichier créé sans instance.\n").unwrap();

    // Lecture simple du fichier créé
    let mut contenu = String::new();
    let mut fichier = File::open("demo.txt").unwrap();
    fichier.read_to_string(&mut contenu).unwrap();
    println!("\nContenu du fichier 'demo.txt' :\n{}", contenu);
}
