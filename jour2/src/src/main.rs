mod ecrire;
mod lire;

use std::io;
use ecrire::Fichier;
use chrono::Local;

struct User{
    nom:String,
    secu: String
}

fn main() -> io::Result<()> {
    // Appel de la méthode de la structure Fichier
    let fichier = Fichier::creer(
        "test2.txt",
        "Fichier créé avec la structure Fichier !
        fait par Julian Trifunovic B3 ESGI SI !!!! \n",
    );

    fichier.ecrire()?; // Ecrit dans le fichier et gère potentiellement l'erreur
    fichier.lire()?; // Lit le contenu du fichier et gère potentiellement l'erreur

    // Affiche la date et l'heure actuelles
    let now = Local::now();
    println!("Date et heure actuelles : {} \n", now.format("%d-%m-%Y %H:%M:%S"));
    

//  Notions d'ownership ( propriété)  et membership ( appartenance à une structure ) => pour garantir 
// la sécurité mémoire 

    // 1.Ownership : 
        // chaque valeur a un propriétaire unique, responsable de libérer la mémoire lorsqu'elle sort du scop
        // quand le propriétaire est déplacé, l'ancien propriétaire ne peut plus y accéder
        // quand le propriétaire sort su scoê, la valeur est automatiquement libérée

    // exemple
        let prenom = String::from("Julian"); // prenom est proprietaire de la String
        let secu = String::from("20040304"); // secu est proprietaire de la String
    // avec clone
        let prenom2 = prenom.clone();
        greetings(prenom); // propriétaire est transféré à la fontion greetings()
    // println!("{}",prenom); //   non Erreur : ownership déplacé !!!!

        println!(" clone de prenom : {}",prenom2);


    // 2 avec emprunt & 
        greetings2(&secu); //  emprunt immuable 
        println!("{}", secu); // oui pas d'Erreur 
    
    // 3  Membership ( Appartenance à une structure )
    //   décrit quelles sont les données contenues dans une structure Struct 

    // exemple : 

    let user = User{
        nom: String::from("Julian"),
        secu: String::from("2004030400007 55")
    };
    
    display(&user);

    Ok(())
}

fn display(u : &User){

    println!("Nom {},  secu : {}",u.nom, u.secu);
}


fn greetings(msg:String){
    println!("Hello  mister, {}",msg);
}

// avec emprunt & 
fn greetings2(msg:&String){
    println!("Hello  mister, {}",msg);
}




// A faire:     créer une structure Fichier et implémenter une fonction qui crée un fichier
                // et qui prend en paramètre le nom de fichier 
                // ecrire.rs 