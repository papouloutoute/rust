// lecture à partir d'un fichier
// dans notre cas on utilise Read et BufReader

use std::fs::File;
use std::io::{self, BufReader, Read};

use crate::ecrire::Fichier; // Corrigez si l'import/module diffère

impl Fichier {
    /// Lit le contenu du fichier
    pub fn lire(&self) -> io::Result<String> {
        let fichier = File::open(&self.nom)?;
        let mut lecteur = BufReader::new(fichier);
        let mut contenu = String::new();
        lecteur.read_to_string(&mut contenu)?;
        println!("Contenu du fichier {} :\n \n{}", self.nom, contenu);
        Ok(contenu)
    }
}

